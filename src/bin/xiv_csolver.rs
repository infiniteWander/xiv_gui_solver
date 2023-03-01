// #![warn(missing_docs,unsafe_code,unstable_features,)]
// #![feature(no_coverage)]

use clap::Parser;
use std::time::Instant;
use xiv_csolver_lib;

#[derive(Debug)]
struct CustomError(String);

fn main() {
    let args = xiv_csolver_lib::io::Args::parse();
    let params = xiv_csolver_lib::io::Parameters::from_args(&args);

    // Start timer
    let now = Instant::now();
    println!("Solving...");

    // Solve from config
    let (recipe, stats) =
        xiv_csolver_lib::load_from_config(&args.recipe_name, &args.file_name, &args.character_name);
    let results = xiv_csolver_lib::solve_craft(recipe, stats, params);

    // Stop timer
    let t_final = now.elapsed().as_millis();

    // Show best result depending on selected value
    match results {
        None => {
            println!(
                "[Error] No solutions found for craft '{}' with crafter '{}' (ms)",
                args.recipe_name, args.character_name
            );
            return;
        }
        Some(ref res) => {
            // Show best results
            if args.verbose > 0 {
                println!("[Final] {} results were found:", res.len());
                #[cfg(feature = "verbose")]
                if args.verbose > 1 {
                    xiv_csolver_lib::print_routes(&results);
                }
            }
        }
    }
    println!("\n > SOLUTION [Least steps] <");
    xiv_csolver_lib::find_fast_route(&results)
        .unwrap()
        .pretty_print();
    println!("\n > SOLUTION [Most durability] <");
    xiv_csolver_lib::find_safe_route(&results)
        .unwrap()
        .pretty_print();
    println!("\n > SOLUTION [Most quality] < ");
    xiv_csolver_lib::find_quality_route(&results)
        .unwrap()
        .pretty_print();

    // #[cfg(feature = "verbose")]
    // if params.verbose>2{
    //     println!("[F] Top routes {:?}",results);
    // }

    // Wait for user input
    println!(
        "\nProgram finished successfully in {}ms\nPress enter to exit...",
        t_final
    );
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    ()
}
