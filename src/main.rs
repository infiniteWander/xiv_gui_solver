// #![warn(missing_docs,unsafe_code,unstable_features,)]
#[allow(dead_code)]

use clap::Parser;
mod utils;

#[derive(Debug)]
struct CustomError(String);

fn main() {
    let args = utils::Args::parse();
    let params = utils::Parameters{
        depth: args.depth,
        threads: args.threads,
        verbose: args.verbose,
    };

    let (recipe,stats) = utils::load_from_config(&args.recipe_name, &args.file_name, &args.character_name);
    utils::solve_craft(recipe,stats,params);
    
    ()
}
