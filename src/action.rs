#[allow(dead_code)]

use std::fmt::Debug;
use lazy_static::lazy_static;
use crate::craft::Craft;
use crate::specs::{Buff, Success};

pub struct Action {
    pub name: String,
    pub dur: u32,
    pub cp: u32,
    pub progress: u32,
    pub quality: u32,
    pub buff: Option<(Buff, u8)>,
    pub short_name: String,
}

pub struct ActionBuilder {
    action: Action,
}

impl ActionBuilder {
    pub fn new(name: &str) -> Self {
        let mut short_name = name.to_string();
        short_name = short_name.replace("II", "2");
        short_name.retain(|c| !c.is_whitespace() && c != '\'');
        short_name[0..1].make_ascii_lowercase();
        if name == "Basic Synthesis" { short_name = "basicSynth".to_string(); }
        if name == "Groundwork" { short_name = "groundwork".to_string(); }
        if name == "Careful Synthesis" { short_name = "carefulSynthesis".to_string(); }

        Self {
            action: Action {
                name: name.to_string(),
                dur: 10,
                cp: 0,
                progress: 0,
                quality: 0,
                buff: None,
                short_name,
            }
        }
    }
    pub fn dur(mut self, dur: u32) -> Self {
        self.action.dur = dur;
        self
    }
    pub fn cp(mut self, cp: u32) -> Self {
        self.action.cp = cp;
        self
    }
    pub fn progress(mut self, progress: u32) -> Self {
        self.action.progress = progress;
        self
    }
    pub fn quality(mut self, quality: u32) -> Self {
        self.action.quality = quality;
        self
    }
    pub fn buff(mut self, buff: Option<(Buff, u8)>) -> Self {
        self.action.buff = buff;
        self
    }
    pub fn build(self) -> Action {
        self.action
    }
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.cp == other.cp && self.dur == other.dur && self.progress == other.progress && self.quality == other.quality
    }
}

impl Action {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_cp_cost(&self, _craft: &Craft) -> u32 {
        if self == &ACTIONS.standard_touch && _craft.buffs.basic_touch > 0 { return 18; };
        if self == &ACTIONS.advanced_touch && _craft.buffs.standard_touch > 0 { return 18; };
        self.cp
    }
    pub fn get_durability_cost(&self, _craft: &Craft) -> u32 {
        if self == &ACTIONS.masters_mend { return 30; }
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
    pub fn get_buff(&self) -> Option<(Buff, u8)> {
        self.buff
    }
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


pub struct ActionList {
    pub muscle_memory: Action,
    pub reflect: Action,
    pub basic_synthesis: Action,
    pub careful_synthesis: Action,
    pub groundwork: Action,
    pub prudent_synthesis: Action,
    pub delicate_synthesis: Action,
    pub basic_touch: Action,
    pub standard_touch: Action,
    pub byregot_blessing: Action,
    pub prudent_touch: Action,
    pub preparatory_touch: Action,
    pub advanced_touch: Action,
    pub trained_finesse: Action,
    pub masters_mend: Action,
    pub waste_not: Action,
    pub waste_not_ii: Action,
    pub manipulation: Action,
    pub veneration: Action,
    pub great_strides: Action,
    pub innovation: Action,
}

impl Default for ActionList {
    fn default() -> Self {
        Self {
            muscle_memory: ActionBuilder::new("Muscle Memory").cp(6).progress(300).buff(Some((Buff::MuscleMemory, 5))).build(),
            reflect: ActionBuilder::new("Reflect").cp(6).quality(100).buff(Some((Buff::InnerQuiet, 1))).build(),

            basic_synthesis: ActionBuilder::new("Basic Synthesis").progress(120).build(),
            careful_synthesis: ActionBuilder::new("Careful Synthesis").cp(7).progress(180).build(),
            groundwork: ActionBuilder::new("Groundwork").cp(18).dur(20).progress(360).build(),
            prudent_synthesis: ActionBuilder::new("Prudent Synthesis").cp(18).dur(5).progress(180).build(),
            delicate_synthesis: ActionBuilder::new("Delicate Synthesis").cp(32).progress(100).quality(100).build(),

            basic_touch: ActionBuilder::new("Basic Touch").cp(18).quality(100).buff(Some((Buff::BasicTouch, 1))).build(),
            standard_touch: ActionBuilder::new("Standard Touch").quality(125).cp(32).buff(Some((Buff::StandardTouch, 1))).build(),
            byregot_blessing: ActionBuilder::new("Byregot's Blessing").cp(24).quality(100).buff(Some((Buff::InnerQuiet, 0))).build(),
            prudent_touch: ActionBuilder::new("Prudent Touch").cp(25).dur(5).quality(100).build(),
            preparatory_touch: ActionBuilder::new("Preparatory Touch").cp(40).dur(20).quality(200).buff(Some((Buff::InnerQuiet, 1))).build(),
            advanced_touch: ActionBuilder::new("Advanced Touch").quality(150).cp(46).build(),
            trained_finesse: ActionBuilder::new("Trained Finesse").cp(32).quality(100).dur(0).build(),

            masters_mend: ActionBuilder::new("Master's Mend").cp(88).build(),
            waste_not: ActionBuilder::new("Waste Not").cp(56).buff(Some((Buff::WasteNot, 4))).build(),
            waste_not_ii: ActionBuilder::new("Waste Not II").cp(98).buff(Some((Buff::WasteNot, 8))).build(),
            manipulation: ActionBuilder::new("Manipulation").cp(96).buff(Some((Buff::Manipulation, 8))).build(),
            veneration: ActionBuilder::new("Veneration").cp(18).buff(Some((Buff::Veneration, 4))).build(),
            great_strides: ActionBuilder::new("Great Strides").cp(32).buff(Some((Buff::GreatStrides, 3))).build(),
            innovation: ActionBuilder::new("Innovation").cp(18).buff(Some((Buff::Innovation, 4))).build(),
        }
    }
}

lazy_static! {
pub static ref ACTIONS: ActionList = ActionList::default();
}