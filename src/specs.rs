//! This modules instanciate all the tools needed to load and use the solver

use std::fmt::Debug;
use strum_macros::EnumIter;

/// The recipe to craft
/// On some websites the fields are instead called
/// [Progress, quality, durability, progress difficulty, quality difficulty, extra progress difficulty, extra quality difficulty]
#[derive(Debug, Clone, Copy)]
pub struct Recipe {
    /// The recipe durability (also called solidity) (35, 40, 70 or 80 usually)
    pub durability: u32,
    /// The total recipe progress needed
    pub progress: u32,
    /// The total recipe quality
    pub quality: u32,
    /// How much the progress is hard to set,
    /// it's an (hidden) extra progress multiplier,
    /// inflicting a -10% to -30% to starred recipes
    /// Also called `progress_difficulty`
    pub progress_divider: u32,
    /// The same (hidden) difficulty multiplier, for quality
    /// Also called `progress_difficulty`
    pub quality_divider: u32,
    /// Modifies how much the base stats are affecting the craft progress
    /// Also called `progress_extra_difficulty`
    pub progress_modifier: u32,
    /// Modifies how much the base stats are affectif the craft quality
    /// Also called `quality_extra_difficulty`
    pub quality_modifier: u32,
}

/// The craftsman stats
#[derive(Debug, Clone, Copy)]
pub struct Stats {
    /// The craftsmanship, influences the progress
    pub craftsmanship: u32,
    /// The control, inluences the quality
    pub control: u32,
    /// The amount of cp, ressource used for actions
    pub max_cp: u32,
}

/// The buffs available for the crafter
#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
pub enum Buff {
    /// Gain a stack of Inner Quiet with every increase in quality, up to a maximum of 10.
    /// Grants a 10% bonus to the efficiency of Touch actions for each stack.
    InnerQuiet,
    /// Reduces loss of durability by 50%.
    WasteNot,
    /// Increases the efficiency of next Touch action by 100%.
    GreatStrides,
    /// Increases efficiency of Touch actions by 50%.
    Innovation,
    /// Increases efficiency of Synthesis actions by 50%.
    Veneration,
    /// Restores 5 points of durability after each step.
    Manipulation,
    /// Increases progress.
    ///     Efficiency: 300%
    ///     Success Rate: 100%
    ///     Additional Effect: Efficiency of your next Synthesis action is increased by 100%
    ///     Available only on the first step.
    ///     Additional effect is active for five steps.
    MuscleMemory,
    // v Hidden v
    /// (Hidden) Combo with Standart touch costs 18 cp.
    BasicTouch,
    /// (Hidden) Combo with Advanced touch costs 18 cp.
    StandardTouch,
    /// (Hidden) Do nothing for one step.
    Observe,
}

/// Keeps track of all the durations remaining on all buffs
/// Also called buff tracker in the docs
#[derive(Debug, Clone, Copy)]
pub struct BuffState {
    /// Caps at 10 for a 100% extra efficiency in quality increase
    pub inner_quiet: u8,
    /// Lasts for four steps if Waste not was used, height if Waste Not II was.
    pub waste_not: u8,
    /// Effect active for three steps, dissipates after a quality action is used.
    pub great_strides: u8,
    /// Lasts four steps
    pub innovation: u8,
    /// Lasts four steps.
    pub veneration: u8,
    /// Lasts height steps
    pub manipulation: u8,
    /// Lasts five steps, dissipate after a progression action is used.
    pub muscle_memory: u8,
    /// Lasts one step
    pub basic_touch: u8,
    /// Lasts one step
    pub standard_touch: u8,
    /// Lasts one step
    pub observe: u8,
}

impl BuffState {
    /// Default state of buffs (all at zero) as per the beggining of craft
    pub fn default() -> Self {
        Self {
            inner_quiet: 0,
            waste_not: 0,
            great_strides: 0,
            innovation: 0,
            veneration: 0,
            manipulation: 0,
            muscle_memory: 0,
            basic_touch: 0,
            standard_touch: 0,
            observe: 0,
        }
    }

    /// Remove the given buff from the buff tracker
    pub fn remove(&mut self, buff: Buff) {
        match buff {
            Buff::InnerQuiet => self.inner_quiet = 0,
            Buff::WasteNot => self.waste_not = 0,
            Buff::GreatStrides => self.great_strides = 0,
            Buff::Innovation => self.innovation = 0,
            Buff::Veneration => self.veneration = 0,
            Buff::Manipulation => self.manipulation = 0,
            Buff::MuscleMemory => self.muscle_memory = 0,
            Buff::BasicTouch => self.basic_touch = 0,
            Buff::StandardTouch => self.standard_touch = 0,
            Buff::Observe => self.observe = 0,
        }
    }

    /// Apply the given buff to the buff tracker for `value` steps
    /// In the case of Inner Quiet, it inseads adds one to the stack counter
    pub fn apply(&mut self, buff: Buff, value: u8) {
        match buff {
            Buff::InnerQuiet => {
                if value == 0 {
                    self.inner_quiet = 0;
                } else {
                    self.inner_quiet += value;
                }
            }
            Buff::WasteNot => self.waste_not = value,
            Buff::GreatStrides => self.great_strides = value,
            Buff::Innovation => self.innovation = value,
            Buff::Veneration => self.veneration = value,
            Buff::Manipulation => self.manipulation = value,
            Buff::MuscleMemory => self.muscle_memory = value,
            Buff::BasicTouch => self.basic_touch = value,
            Buff::StandardTouch => self.standard_touch = value,
            Buff::Observe => self.observe = value,
        }
    }

    /// Tick downs the remaining duration of all buffs, if up
    pub fn tick(&mut self) {
        if self.waste_not > 0 {
            self.waste_not -= 1;
        }
        if self.great_strides > 0 {
            self.great_strides -= 1;
        }
        if self.innovation > 0 {
            self.innovation -= 1;
        }
        if self.veneration > 0 {
            self.veneration -= 1;
        }
        if self.manipulation > 0 {
            self.manipulation -= 1;
        }
        if self.muscle_memory > 0 {
            self.muscle_memory -= 1;
        }
        if self.basic_touch > 0 {
            self.basic_touch -= 1;
        }
        if self.standard_touch > 0 {
            self.standard_touch -= 1;
        }
    }
}

/// State the craft is in, used to keep track of wether the craft
/// is to be worked on, added to the success vector or discarded
#[derive(Debug, Clone, PartialEq)]
pub enum Success {
    /// Pending success and therefor still ongoing
    Pending,
    /// Finised, not ongoing and finished without breaking (Doens't take quality into account)
    Success,
    /// Finised because of lack of remaining durability
    Failure,
}

// Test unit
#[cfg(test)]
mod tests {
    // Test default values
    use pretty_assertions::assert_eq;
    use strum::IntoEnumIterator;

    #[test]
    pub fn test_structs() {
        format!(
            "{:?}",
            crate::specs::Recipe {
                durability: 0,
                progress: 0,
                progress_divider: 0,
                progress_modifier: 0,
                quality: 0,
                quality_divider: 0,
                quality_modifier: 0,
            }
            .clone()
        );

        format!(
            "{:?}",
            crate::specs::Stats {
                craftsmanship: 0,
                control: 0,
                max_cp: 0,
            }
            .clone()
        );

        let iq = crate::specs::Buff::InnerQuiet;
        assert_eq!(crate::specs::Buff::InnerQuiet, iq);
        for buff in crate::specs::Buff::iter() {
            format!("{:?}", buff);
        }
        format!("{:?}", crate::specs::BuffState::default().clone());

        let sc = crate::specs::Success::Pending;
        assert!(crate::specs::Success::Pending == sc);
        assert!(crate::specs::Success::Success != sc);
        format!("{:?}", sc);
    }

    #[test]
    pub fn test_buffstate() {
        let mut bbs = crate::specs::BuffState::default();
        for buff in crate::specs::Buff::iter() {
            bbs.apply(buff, 2);
        }
        assert_eq!(bbs.inner_quiet, 2);
        assert_eq!(bbs.waste_not, 2);
        assert_eq!(bbs.great_strides, 2);
        assert_eq!(bbs.innovation, 2);
        assert_eq!(bbs.veneration, 2);
        assert_eq!(bbs.manipulation, 2);
        assert_eq!(bbs.muscle_memory, 2);
        assert_eq!(bbs.basic_touch, 2);
        assert_eq!(bbs.standard_touch, 2);
        assert_eq!(bbs.observe, 2);
        bbs.apply(crate::specs::Buff::InnerQuiet, 0);
        bbs.apply(crate::specs::Buff::Innovation, 5);
        assert_eq!(bbs.inner_quiet, 0);
        assert_eq!(bbs.innovation, 5);
        bbs.apply(crate::specs::Buff::InnerQuiet, 2);
        bbs.apply(crate::specs::Buff::InnerQuiet, 1);
        assert_eq!(bbs.inner_quiet, 3);
        bbs.tick();
        assert_eq!(bbs.inner_quiet, 3);
        assert_eq!(bbs.innovation, 4);
        for buff in crate::specs::Buff::iter() {
            bbs.remove(buff);
        }
        assert_eq!(bbs.inner_quiet, 0);
        assert_eq!(bbs.innovation, 0);
    }
}
