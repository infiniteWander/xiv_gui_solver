// #![warn(missing_docs,unsafe_code,unstable_features,)]
use clap::Parser;
use xiv_craft_solver;

#[derive(Debug)]
struct CustomError(String);

fn main() {
    let args = xiv_craft_solver::Args::parse();
    let params = xiv_craft_solver::Parameters{
        depth: args.depth,
        threads: args.threads,
        verbose: args.verbose,
    };

    let (recipe,stats) = xiv_craft_solver::load_from_config(&args.recipe_name, &args.file_name, &args.character_name);
    xiv_craft_solver::solve_craft(recipe,stats,params);
    
    ()
}