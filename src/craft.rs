//! Module containing everything to create a craft and all methods to get the current state of the craft
//!
//! A craft represent the state of the recipe with a vector of actions applied to it.
//! Applying actions to it will make the craft progress, while branching and trying other actions
//! will be done by cloning
//!

use crate::action::Action;
use crate::specs::{Buff, BuffState, Recipe, Stats, Success};
use crate::Parameters;
use std::fmt::{Debug, Display, Formatter};

/// The craft structure, note that there is no standard way of backtracking
/// and a traditional branching method is applied when solving.
#[derive(Clone)]
pub struct Craft<'a> {
    /// The recipe used, contains all needed information on the difficulty and target
    pub recipe: Recipe,
    /// The stats of the crafter
    pub stats: Stats,
    /// The buffs currently running
    pub buffs: BuffState,
    /// The step count, aka number of steps done from the beggining of the craft
    pub step_count: u32,
    /// The remaining durability
    pub durability: i32,
    /// The current progression
    pub progression: u32,
    /// The current quality
    pub quality: u32,
    /// The cp currently remaining
    pub cp: i32,
    /// Wether the craft has succeded, is pending sucess, or failed
    pub success: Success,
    /// The vector of action already applied, structs are used for the vector since
    /// static references are used and therefore take no additional memory at runtime
    pub actions: Vec<&'a Action>,
    /// The parameters used for the craft
    /// TODO: The usage of a non reference is quite an oversight and should be changed ASAP
    pub args: Parameters,
}

impl<'a> Craft<'a> {
    /// Create a new craft with a recipe, stats and parameters
    ///
    pub fn new(recipe: Recipe, stats: Stats, params: Parameters) -> Craft<'a> {
        Self {
            recipe,
            stats,
            buffs: BuffState::default(),
            step_count: 0,
            durability: recipe.durability as i32,
            progression: 0,
            quality: 0,
            cp: stats.max_cp as i32,
            success: Success::Pending,
            actions: Vec::new(),
            args: params,
        }
    }

    /// How much progression will be done by an action with 100 efficiency and no buffs
    /// This function is mainly used to know wether or not the craft will finish in
    /// 1, 1.2 or 2.0 actions.
    pub fn get_base_progression(&self) -> u32 {
        let base_value = (self.stats.craftsmanship as f64 / 10.0)
            / (self.recipe.progress_divider as f64 / 100.0)
            + 2.0;
        (base_value * (self.recipe.progress_modifier as f64 / 100.0) as f64).floor() as u32
    }
    /// How much quality will the crafter generate with one action with 100 efficiency and no buffs
    /// This function is usually called by [`run_action`](crate::craft::Craft::run_action)
    /// wich will then apply all needed buff and progress the craft
    pub fn get_base_quality(&self) -> u32 {
        let base_value = (self.stats.control as f64 / 10.0)
            / (self.recipe.quality_divider as f64 / 100.0)
            + 35.0;
        (base_value * (self.recipe.quality_modifier as f64 / 100.0) as f64).floor() as u32
    }

    /// Add an action to the current craft
    /// This method will:
    /// - Check wether the action can be used
    /// - Increment the step count
    /// - Decrease cp
    /// - Decrease or increase durability
    /// - Increase progression
    /// - "Tick" buffs (substract 1 from their remaining time) and remove them if consumed
    /// - Add the used action to the vec of actions used
    pub fn run_action(&mut self, action: &'a Action) -> &mut Craft<'a> {
        if !action.can_use(self) {
            self.success = Success::Failure;
            return self;
        }
        self.step_count += 1;
        self.cp -= action.get_cp_cost(self) as i32;
        self.durability -= action.get_durability_cost(self) as i32;
        self.progression += (action.get_progress(self) as f64
            * (self.get_base_progression() as f64 / 100.0))
            .floor() as u32;
        self.quality += (action.get_quality(self) as f64 * (self.get_base_quality() as f64 / 100.0))
            .floor() as u32;
        if self.progression >= self.recipe.progress {
            self.success = Success::Success;
        }
        if self.durability <= 0 {
            self.success = Success::Failure;
        }
        if self.buffs.manipulation > 0 {
            self.durability += 5;
        }
        if action.quality > 0 {
            self.buffs.inner_quiet += 1;
        }
        self.buffs.tick();
        if action.progress > 0 {
            self.buffs.remove(Buff::MuscleMemory);
        }
        if action.quality > 0 {
            self.buffs.remove(Buff::GreatStrides);
        }
        if let Some((buff, duration)) = action.get_buff() {
            self.buffs.apply(buff, duration);
        }
        if self.durability > self.recipe.durability as i32 {
            self.durability = self.recipe.durability as i32;
        }
        if self.buffs.inner_quiet > 10 {
            self.buffs.inner_quiet = 10;
        }

        self.actions.push(action);
        return self;
    }

    // Assumes the craft is finished and tries to add the last actions
    // pub fn add_last_actions(&mut self)->&Self{
    //     let arg = (self.recipe.progress as f32 - self.progression as f32) / self.get_base_progression() as f32;
    //     self.durability+=10;
    //     if 0.0 < arg && arg < 1.2 { self.run_action(&ACTIONS.basic_synthesis);}
    //     if 1.2 <= arg && arg < 1.8 { self.cp+=7; self.run_action(&ACTIONS.careful_synthesis); }
    //     if 1.8 <= arg && arg < 2.0 {
    //         self.cp+=12;
    //         self.run_action(&ACTIONS.observe);
    //         self.run_action(&ACTIONS.focused_synthesis);
    //     }
    //     self
    // }
}

impl<'a> Debug for Craft<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut binding = f.debug_struct("");
        binding
            .field("step_count", &self.step_count)
            .field("progression", &format!("{:?}/{:?}", &self.progression, &self.recipe.progress))
            .field("quality", &format!("{:?}/{:?}", &self.quality, &self.recipe.quality))
            .field("durability", &format!("{:?}/{:?}", &self.durability, &self.recipe.durability))
            // .field("cp", &format!("{:?}/{:?}", &self.cp, &self.stats.max_cp))
        ;
        binding.field("actions", &self.actions);
        binding.finish()
    }
}

impl<'a> Display for Craft<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("Craft")
            .field("step_count", &self.step_count)
            .field(
                "progression",
                &format!("{:?}/{:?}", &self.progression, &self.recipe.progress),
            )
            .field(
                "quality",
                &format!("{:?}/{:?}", &self.quality, &self.recipe.quality),
            )
            .field(
                "durability",
                &format!("{:?}/{:?}", &self.durability, &self.recipe.durability),
            )
            .field("cp", &format!("{:?}/{:?}", &self.cp, &self.stats.max_cp))
            .finish()
    }
}


#[cfg(test)]
mod test{
    // Struct Test
    #[test]
    pub fn struct_tests(){
        
    }


}