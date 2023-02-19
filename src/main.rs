use crate::craft::Craft;
use crate::libs::{Args,};
use crate::specs::{Recipe, Stats};

use clap::Parser;

mod specs;
pub mod action;
pub mod craft;
mod solver;
pub mod libs;

#[derive(Debug)]
struct CustomError(String);

fn main() {
    // Get args
    let args = Args::parse();

    // Load the craft with given arguments
    let craft = libs::load_config(&args);
    
    libs::solve_craft(craft,&args);
    
    ()
}
