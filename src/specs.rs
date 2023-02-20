use std::fmt::{Debug};
use strum_macros::EnumIter;


#[derive(Debug, Clone, Copy)]
pub struct Recipe {
    pub durability: u32,
    pub progress: u32,
    pub quality: u32,
    pub progress_divider: u32,
    pub quality_divider: u32,
    pub progress_modifier: u32,
    pub quality_modifier: u32,
}


#[derive(Debug, Clone, Copy)]
pub struct Stats {
    pub craftsmanship: u32,
    pub control: u32,
    pub max_cp: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
pub enum Buff {
    InnerQuiet,
    WasteNot,
    GreatStrides,
    Innovation,
    Veneration,
    Manipulation,
    MuscleMemory,
    // v Hidden v
    BasicTouch,
    StandardTouch,
    // Observe,
}

#[derive(Debug, Clone, Copy)]
pub struct BuffState {
    pub inner_quiet: u8,
    pub waste_not: u8,
    pub great_strides: u8,
    pub innovation: u8,
    pub veneration: u8,
    pub manipulation: u8,
    pub muscle_memory: u8,
    pub basic_touch: u8,
    pub standard_touch: u8,
}

impl BuffState {
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
        }
    }

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
        }
    }

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
        }
    }

    pub fn tick(&mut self) {

        if self.waste_not > 0 { self.waste_not -= 1; }
        if self.great_strides > 0 { self.great_strides -= 1; }
        if self.innovation > 0 { self.innovation -= 1; }
        if self.veneration > 0 { self.veneration -= 1; }
        if self.manipulation > 0 { self.manipulation -= 1; }
        if self.muscle_memory > 0 { self.muscle_memory -= 1; }
        if self.basic_touch > 0 { self.basic_touch -= 1; }
        if self.standard_touch > 0 { self.standard_touch -= 1; }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Success {
    Pending,
    Success,
    Failure,
}
