//! # Solver bin for FFXIV Crafting in Rust
//!
//! This project strives to provide a fast, accurate and reliable way to craft
//! in FFXIV and obtain HQ results
//!
//! This project also provides bindings for python to allow hooking it to any form of better GUI/CLI
//!
//! The project is (for now) self-hosted and hosted on [Github](https://github.com/Dandaedre/xiv_csolver)
//!
//! ## Getting started
//!
//! ### Development
//!  
//! #### Rust
//! To set up the whole development environment, it is recommended to use cargo-make
//!  
//! Doing so will allow you custom commands to
//!
//! ```sh
//! cargo make rebuild
//! ```
//! To rebuild the whole project
//!
//! ```sh
//! cargo make release
//! ```
//! To generate the release files, bins and libs, package them and make them ready to release
//!
//! #### Python
//!
//! It is really recommended to set up a venv to handle the python development
//!
//! All bindings and the explanation of how to use them can be found in the [`io`](crate::io) module.
//!
//! ### Features
//!
//! Use these by running `cargo run/build --features <name of feature>`
//!
//! - ``no_python`` : Don't build the lib & python bindings (WIP, not fully implemented)
//! - ``fast`` : Don't allow some under-optimisation to run (saving global run-time)
//! - ``verbose`` : Allow for more debug messages (note that -v(vv) must still be passed to activate them)
//!
//! ## Regenerating the API Bindings
//!
//! The python bindings are generated using maturin
//!
//! ```sh
//! cargo install maturin
//! maturin develop
//! ```
//! ## Solving and bias
//!
//! The solving algorithm used has a very strong bias toward a particular method:
//!
//! - **Step 1** : Try to push the progression of the craft one step before being finishable
//! - **Step 2** : Try to increase the quality as much as possible.
//! - **Step 3** : Finish the craft
//!
//! To account for a lot of possibilities, step one generates all methods to try and finish the craft
//! and step two tests all methods to up the quality until all crafting points are depleted
//! Doing so ensures by brute force that all permutations are thoroughly tested.
//!
//! While this method works and is very likely the best method for the current patch,
//! a rework of the crafting actions could very well make this method subpar or at least
//! on par with another method.
//!
//! In layman's term, only the groundwork opening is tested and no delicate synthesis spam is tested.
//! However, for now, these two crafting techniques remain subpar and discarding them comes at no
//! loss for the crafter.
//!
//! Others algorithm exists and could very well cover all other basis at the expense of runtime.
//! We have chosen not to implement them as they (for now) would be very unlikely to uncover anything
//! meaningful.
//!
//! ### Leveling
//!
//! We have (for now at least) no intention to release a leveling feature to the crafting solver
//! However, selecting some spells and buffs and have them disabled could be added as a feature
//! hence making a "leveling" option
//!
//! If it is ever released, this feature is likely to be heavily reliant on the python GUI.
//!
//!
//! ## Contributing
//!
//! All form of contributions are welcome !
//!
//! While this project was originally a collab and didn't strive to encompass every need
//! of a modern FFXIV crafter, more polish and finish is always welcome.
//!
//! ## License
//!
//! The original code was released without a license by [RikaKagurasaka](https://github.com/RikaKagurasaka/xiv_craft_solver)
//!
//! All this fork's code is released under [Apache-2.0](http://www.apache.org/licenses/LICENSE-2.0)

#![warn(missing_docs)]

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
