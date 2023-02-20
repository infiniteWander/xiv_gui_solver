// // #![warn(missing_docs,unsafe_code,unstable_features,)]
// #[allow(dead_code)]

// use clap::Parser;
// use xiv_craft_solver_lib;

// #[derive(Debug)]
// struct CustomError(String);

// fn main() {
//     let args = xiv_craft_solver_lib::Args::parse();
//     let params = xiv_craft_solver_lib::Parameters{
//         depth: args.depth,
//         threads: args.threads,
//         verbose: args.verbose,
//     };

//     let (recipe,stats) = xiv_craft_solver_lib::load_from_config(&args.recipe_name, &args.file_name, &args.character_name);
//     xiv_craft_solver_lib::solve_craft(recipe,stats,params);
    
//     ()
// }
fn main() {
    unimplemented!();
}