// #![warn(missing_docs,unsafe_code,unstable_features,)]
use crate::libs::Parameters;
use crate::libs::load_from_config;
use clap::Parser;
use crate::libs::Args;
use crate::craft::Craft;
use crate::specs::{Recipe, Stats};


mod specs;
pub mod action;
pub mod craft;
mod solver;
pub mod libs;

#[derive(Debug)]
struct CustomError(String);

fn main() {
    let args = Args::parse();
    let params = Parameters{
        depth: args.depth,
        threads: args.threads,
        verbose: args.verbose,
    };

    let (recipe,stats) = load_from_config(&args.recipe_name, &args.file_name, &args.character_name);
    libs::solve_craft(recipe,stats,params);
    
    ()
}
