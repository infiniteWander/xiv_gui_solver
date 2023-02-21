// #![warn(missing_docs,unsafe_code,unstable_features,)]
use std::time::{Instant};
use clap::Parser;
use xiv_craft_solver;

#[derive(Debug)]
struct CustomError(String);

fn main() {
    let args = xiv_craft_solver::io::Args::parse();
    let params = xiv_craft_solver::io::Parameters{
        depth: args.depth,
        threads: args.threads,
        verbose: args.verbose,
    };

    // Start timer
    let now = Instant::now();
    println!("Solving...");

    // Solve from config
    let (recipe,stats) = xiv_craft_solver::load_from_config(&args.recipe_name, &args.file_name, &args.character_name);
    let results = xiv_craft_solver::solve_craft(recipe,stats,params);
    
    // Stop timer
    let t_final = now.elapsed().as_millis();
    
    // Show best results
    #[cfg(feature = "verbose")]
    if args.verbose>0{
        print_routes(results);
    }

    // Show best result depending on selected value
    println!("\n > SOLUTION [Least steps] <");
    xiv_craft_solver::find_fast_route(&results).unwrap().pretty_print();
    println!("\n > SOLUTION [Most durability] <");
    xiv_craft_solver::find_safe_route(&results).unwrap().pretty_print();
    println!("\n > SOLUTION [Most quality] < ");
    xiv_craft_solver::find_quality_route(&results).unwrap().pretty_print();

    #[cfg(feature = "verbose")]
    if params.verbose>2{
        println!("[F] Top routes {:?}",results);
    }

    // Wait for user input
    println!("\nProgram finished successfully in {}ms\nPress enter to exit...", t_final);
    let mut input = String::new(); std::io::stdin().read_line(&mut input).unwrap();
    ()
}