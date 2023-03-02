//! Module containing all actions that a crafter can execute
//!
//! All actions are defined as Action, containing all methods to actively use it in a craft
//! while ActionBuilder is used only initally as a lazy_static setup
//!

use crate::craft::Craft;
use crate::specs::{Buff, Success};
use lazy_static::lazy_static;
use std::fmt::Debug;

/// Hold an action for the solver to try using
/// 
/// It will be used heavily to try/discard actions durin craft
pub struct Action {
    /// The action name, in a human readable format, it will be used to generate a short_name
    pub name: String,
    /// The natural durability cost, without buff or reductions
    pub dur: u32,
    /// The cost in cp
    pub cp: u32,
    /// The amount of progression provided in percent (not the actual value added to the craft)
    pub progress: u32,
    /// The amount of quality provided in percent (not the acual value added to the craft)
    pub quality: u32,
    /// The name of the buff given, if any
    pub buff: Option<(Buff, u8)>,
    /// The short name of the action, as used by most crafting simulator (aka: stripped of whitespace)
    pub short_name: String,
}

/// Builds an action based on an action
/// 
/// Made to initialize lazy_static and nothing more
pub struct ActionBuilder {
    action: Action,
}

impl ActionBuilder {
    /// Base creator, takes a name and initialize an empty action with the short_name
    /// and name setup and default values for all other fields
    /// 
    /// # Examples
    /// ```
    /// use xiv_csolver_lib::action::ActionBuilder;
    /// use xiv_csolver_lib::specs::Buff;
    /// ActionBuilder::new("Muscle Memory")
    ///            .cp(6)
    ///            .progress(300)
    ///            .buff(Some((Buff::MuscleMemory, 5)))
    ///            .build();
    /// ```
    pub fn new(name: &str) -> Self {
        let mut short_name = name.to_string();
        short_name = short_name.replace("II", "2");
        short_name.retain(|c| !c.is_whitespace() && c != '\'');
        short_name[0..1].make_ascii_lowercase();
        if name == "Basic Synthesis" {
            short_name = "basicSynth2".to_string();
        }
        if name == "Groundwork" {
            short_name = "groundwork".to_string();
        }
        if name == "Careful Synthesis" {
            short_name = "carefulSynthesis".to_string();
        }

        Self {
            action: Action {
                name: name.to_string(),
                dur: 10,
                cp: 0,
                progress: 0,
                quality: 0,
                buff: None,
                short_name,
            },
        }
    }
    /// Changes the durability consumption of an action
    pub fn dur(mut self, dur: u32) -> Self {
        self.action.dur = dur;
        self
    }

    /// Changes the crafting points (cp) consumption of an action
    pub fn cp(mut self, cp: u32) -> Self {
        self.action.cp = cp;
        self
    }
    /// Changes the amount of progress of an action
    pub fn progress(mut self, progress: u32) -> Self {
        self.action.progress = progress;
        self
    }
    /// Changes the amount of quality of an action
    pub fn quality(mut self, quality: u32) -> Self {
        self.action.quality = quality;
        self
    }
    /// Makes an action add a buff for certain number of actions
    /// The buffs available are defined in [specs.rs](xiv_csolver::specs::Buff)
    /// 
    /// ``
    /// (self.recipe.progress as f32 - self.progression as f32) / self.get_base_progression() as f32;
    /// ``   
    pub fn buff(mut self, buff: Option<(Buff, u8)>) -> Self {
        self.action.buff = buff;
        self
    }
    /// Finalize the usage of the ActionBuilder and return the action
    /// Once again, all this is for the usage of lazy_static
    pub fn build(self) -> Action {
        self.action
    }
}

/// Custom implementation of the equality operator, it is used to compare actions during the
/// the crafting process
impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.cp == other.cp
            && self.dur == other.dur
            && self.progress == other.progress
            && self.quality == other.quality
    }
}

/// A craft action, after compilation (with lazy_static) 
// no other actions should be created  
impl Action {
    /// Get the printable name of the action
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    /// Get the CP cost, reduced by the eventual chain buffs currently active in the craft
    pub fn get_cp_cost(&self, _craft: &Craft) -> u32 {
        if self == &ACTIONS.standard_touch && _craft.buffs.basic_touch > 0 {
            return 18;
        };
        if self == &ACTIONS.advanced_touch && _craft.buffs.standard_touch > 0 {
            return 18;
        };
        self.cp
    }
    /// Get the durability cost this action would have if used at this stage of the craft
    /// This method accounts for the effects of **master mend** and **waste not**.
    pub fn get_durability_cost(&self, _craft: &Craft) -> u32 {
        if self == &ACTIONS.masters_mend {
            return 30;
        }
        if self.dur == 0 || self.progress == 0 && self.quality == 0 {
            return 0;
        }
        let mut dur = self.dur;
        if _craft.buffs.waste_not > 0 {
            dur /= 2;
        }
        if dur < 5 {
            dur = 5;
        }
        dur
    }
    /// Get the progress effect this action would have if used at this stage of the craft
    /// This method accounts for the effects of **groundwork** under 20 durability,
    /// and the effects of **veneration** and **muscle memory**
    /// # Exemple
    /// ```md
    ///     - Groundwork will return 200 if the durability remaining is above 20
    ///     - Basic Synthesis will return 100 if no buffs are up and 150 with veneration
    /// ```
    pub fn get_progress(&self, _craft: &Craft) -> u32 {
        let mut prog = self.progress as f64;
        if self.dur == 20 && _craft.durability < self.get_durability_cost(_craft) as i32 {
            prog /= 2.0;
        }
        let mut mult = 1.0;
        if _craft.buffs.muscle_memory > 0 {
            mult += 1.0;
        }
        if _craft.buffs.veneration > 0 {
            mult += 0.5;
        }
        (prog * mult).floor() as u32
    }

    /// Get the quality effect this action would have if used at this stage of the craft. 
    /// This method account for the **inner quiet** stacking effect, **innovation** and **great strides**
    pub fn get_quality(&self, _craft: &Craft) -> u32 {
        let mut qual = self.quality as f64;
        if self == &ACTIONS.byregot_blessing {
            qual = (100 + 20 * _craft.buffs.inner_quiet as u32) as f64;
        }
        if self.dur == 20 && _craft.durability < self.get_durability_cost(_craft) as i32 {
            qual /= 2.0;
        }
        let mut mult = 1.0;
        if _craft.buffs.great_strides > 0 {
            mult += 1.0;
        }
        if _craft.buffs.innovation > 0 {
            mult += 0.5;
        }
        let iq_mult = (_craft.buffs.inner_quiet as f64 * 0.1) + 1.0;
        (qual * mult * iq_mult).floor() as u32
    }
    /// Returns the Buff (if any) up at this time of the craft
    pub fn get_buff(&self) -> Option<(Buff, u8)> {
        self.buff
    }
    /// Returns a bool representing if this action **can** be used, note that this does only account
    /// for the game limitations and not viability limitation
    /// 
    /// # Improvements
    /// This function could easily prune useless actions by considering them not available.
    /// Doing so could stop early refresh of buffs and spam of observe and such.
    /// However some of such "optimisations" could just plain hurt the craft by forbidding a *seemingly*
    /// useless action. The "bruteforce" method being easily computatble on a modern computer, such 
    /// optimisations arent warrented and left to aother time.
    pub fn can_use(&self, _craft: &Craft) -> bool {
        if self.get_cp_cost(_craft) > _craft.cp as u32 {
            return false;
        }
        if _craft.success != Success::Pending {
            return false;
        }
        if self == &ACTIONS.byregot_blessing {
            return _craft.buffs.inner_quiet > 0;
        }
        if self == &ACTIONS.muscle_memory || self == &ACTIONS.reflect {
            return _craft.step_count == 0;
        }
        if self == &ACTIONS.trained_finesse {
            return _craft.buffs.inner_quiet >= 10;
        }
        true
    }
}

impl Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

/// The callable actionlist, to be used in conjunction with lazy_static
pub struct ActionList {
    /// Increases progress.
    ///     Efficiency: 300%
    ///     Success Rate: 100%
    ///     Additional Effect: Efficiency of your next Synthesis action is increased by 100%
    ///     Available only on the first step.
    ///     Additional effect is active for five steps.
    pub muscle_memory: Action,
    /// Increases quality.
    ///     Efficiency: 100%
    ///     Success Rate: 100%
    ///     Additional Effect: Increases Inner Quiet stack by one (up to 10)
    ///     Availabe only on the first step.
    pub reflect: Action,
    /// Increases progress.
    ///     Efficiency: 120%
    ///     Success Rate: 100%
    pub basic_synthesis: Action,
    /// Increases progress.
    ///     Efficiency: 120%
    ///     Success Rate: 100%
    pub careful_synthesis: Action,
    /// Increases progress at greater cost to durability.
    /// Efficiency decreases by half when durability is below durability cost.
    ///     Efficiency: 300%
    ///     Success Rate: 100%
    ///     Durability Cost: 20
    pub groundwork: Action,
    /// Increases progress at half the durability cost.
    ///     Efficiency: 180%
    ///     Success Rate: 100%
    ///     Unavailable when Waste Not or Waste Not II is active.
    pub prudent_synthesis: Action,
    /// Increases both progress and quality.
    ///     Efficiency: 100%
    ///     Success Rate: 100%
    pub delicate_synthesis: Action,
    /// Increases quality.
    ///     Efficiency: 100%
    ///     Success Rate: 100%
    pub basic_touch: Action,
    /// Increases quality.
    ///     Efficiency: 125%
    ///     Success Rate: 100%
    ///     Combo action: Basic Touch
    ///     Combo bonus: CP cost reduced to 18
    pub standard_touch: Action,
    /// Increases quality. Inner Quiet effect ends upon use.
    ///     Efficiency: 100% plus 20% for each count of your Inner Quiet stack, up to a maximum of 300%
    ///     Success Rate: 100%
    ///     Requires at least one stack of Inner Quiet.
    pub byregot_blessing: Action,
    /// Increases quality at half the durability cost.
    ///     Efficiency: 100%
    ///     Success Rate: 100%
    ///     Unavailable when Waste Not or Waste Not II are active.
    pub prudent_touch: Action,
    /// Increases quality at a greater cost to durability.
    ///     Additional Effect: Increases Inner Quiet stack by one (up to 11)
    ///     Efficiency: 200%
    ///     Success Rate: 100%
    ///     Durability Cost: 20
    pub preparatory_touch: Action,
    /// Increases quality.
    ///     Efficiency: 150%
    ///     Success Rate: 100%
    ///     Combo action: Standard Touch
    ///     Combo bonus: CP cost reduced to 18
    pub advanced_touch: Action,
    /// Increases quality at no cost to durability.
    ///     Efficiency: 100%
    ///     Success Rate: 100%
    ///     Available only when Inner Quiet stack size is 10.
    pub trained_finesse: Action,
    /// Restores item durability by 30. 
    pub masters_mend: Action,

    // Restores item durability by 60. 
    // pub masters_mend_2: Action,

    /// Reduces loss of durability by 50% for the next four steps. 
    pub waste_not: Action,
    /// Reduces loss of durability by 50% for the next eight steps. 
    pub waste_not_ii: Action,
    /// Restores 5 points of durability after each step for the next eight steps.
    pub manipulation: Action,
    /// Increases efficiency of Synthesis actions by 50% for the next four steps. 
    pub veneration: Action,
    /// Increases the efficiency of next Touch action by 100%.
    /// Effect active for three steps. 
    pub great_strides: Action,
    /// Increases efficiency of Touch actions by 50% for the next four steps.
    pub innovation: Action,

    // Do nothing for one step. 
    //pub observe: Action,
    // Increases progress.
    //     Efficiency: 200%
    //     Success Rate: 50%
    //     Combo Action: Observe
    //     Combo Bonus: Increases success rate to 100%
    //pub focused_synthesis: Action,
}

/// Create all base actions as a default (for lazy_static)
impl Default for ActionList {
    fn default() -> Self {
        Self {
            muscle_memory: ActionBuilder::new("Muscle Memory")
                .cp(6)
                .progress(300)
                .buff(Some((Buff::MuscleMemory, 5)))
                .build(),
            reflect: ActionBuilder::new("Reflect")
                .cp(6)
                .quality(100)
                .buff(Some((Buff::InnerQuiet, 1)))
                .build(),

            basic_synthesis: ActionBuilder::new("Basic Synthesis").progress(120).build(),
            careful_synthesis: ActionBuilder::new("Careful Synthesis")
                .cp(7)
                .progress(180)
                .build(),
            groundwork: ActionBuilder::new("Groundwork")
                .cp(18)
                .dur(20)
                .progress(360)
                .build(),
            prudent_synthesis: ActionBuilder::new("Prudent Synthesis")
                .cp(18)
                .dur(5)
                .progress(180)
                .build(),
            delicate_synthesis: ActionBuilder::new("Delicate Synthesis")
                .cp(32)
                .progress(100)
                .quality(100)
                .build(),

            basic_touch: ActionBuilder::new("Basic Touch")
                .cp(18)
                .quality(100)
                .buff(Some((Buff::BasicTouch, 1)))
                .build(),
            standard_touch: ActionBuilder::new("Standard Touch")
                .quality(125)
                .cp(32)
                .buff(Some((Buff::StandardTouch, 1)))
                .build(),
            byregot_blessing: ActionBuilder::new("Byregot's Blessing")
                .cp(24)
                .quality(100)
                .buff(Some((Buff::InnerQuiet, 0)))
                .build(),
            prudent_touch: ActionBuilder::new("Prudent Touch")
                .cp(25)
                .dur(5)
                .quality(100)
                .build(),
            preparatory_touch: ActionBuilder::new("Preparatory Touch")
                .cp(40)
                .dur(20)
                .quality(200)
                .buff(Some((Buff::InnerQuiet, 1)))
                .build(),
            advanced_touch: ActionBuilder::new("Advanced Touch")
                .quality(150)
                .cp(46)
                .build(),
            trained_finesse: ActionBuilder::new("Trained Finesse")
                .cp(32)
                .quality(100)
                .dur(0)
                .build(),

            masters_mend: ActionBuilder::new("Master's Mend").cp(88).build(),
            waste_not: ActionBuilder::new("Waste Not")
                .cp(56)
                .buff(Some((Buff::WasteNot, 4)))
                .build(),
            waste_not_ii: ActionBuilder::new("Waste Not II")
                .cp(98)
                .buff(Some((Buff::WasteNot, 8)))
                .build(),
            manipulation: ActionBuilder::new("Manipulation")
                .cp(96)
                .buff(Some((Buff::Manipulation, 8)))
                .build(),
            veneration: ActionBuilder::new("Veneration")
                .cp(18)
                .buff(Some((Buff::Veneration, 4)))
                .build(),
            great_strides: ActionBuilder::new("Great Strides")
                .cp(32)
                .buff(Some((Buff::GreatStrides, 3)))
                .build(),
            innovation: ActionBuilder::new("Innovation")
                .cp(18)
                .buff(Some((Buff::Innovation, 4)))
                .build(),
            // Todo: Add back
            //observe: ActionBuilder::new("observe").cp(18).buff(Some((Buff::Observe, 1))).build(),
            //focused_synthesis: ActionBuilder::new("focused_synthesis").cp(18).quality(100).build(),
        }
    }
}

lazy_static! {
    /// Initialize a static pseudo &Craft: ACTIONS 
    pub static ref ACTIONS: ActionList = ActionList::default();
}
