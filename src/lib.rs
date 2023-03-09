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

use crate::craft::Craft;
use crate::io::Parameters;
use crate::python_bindings::{Info, SolverResult};
use crate::specs::{Recipe, Stats};
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

pub mod action;
pub mod craft;
pub mod io;
pub mod python_bindings;
mod solver;
pub mod specs;

/// Solve the craft with given arguments, this functions calls threads and must own it's values
pub fn solve_craft<'a>(
    recipe: Recipe,
    stats: Stats,
    params: Parameters,
) -> Option<(Vec<SolverResult>, Info)> {
    // Load the craft with given arguments
    let craft = Craft::new(recipe, stats, params);

    // Start a threadpool
    let pool = ThreadPool::new(params.threads);

    #[cfg(feature = "verbose")]
    if params.verbose > 0 {
        println!("[P1] Starting phase 1...");
    }

    let phase1_routes = solver::generate_routes_phase1(craft);
    let nb_p1 = phase1_routes.len();

    #[cfg(feature = "verbose")]
    if params.verbose > 0 {
        println!(
            "[P1] Found {} routes, testing them all...",
            phase1_routes.len()
        );
        if params.verbose > 1 {
            for r in &phase1_routes {
                println!(
                    "[P1] {:?} p:{}% q:{}% c:{} d:{}",
                    r.actions,
                    r.progression,
                    r.quality * 100 / r.recipe.quality,
                    r.cp,
                    r.durability
                );
            }
        }
    }

    // Core algorithm, fill all found routes with the best route
    let arc_phase2_routes = Arc::new(Mutex::new(Vec::<Craft>::new()));

    for route in phase1_routes {
        let _phase2_routes = Arc::clone(&arc_phase2_routes);

        pool.execute(move || {
            if let Some(mut _route) = solver::generate_routes_phase2(route) {
                let mut shared = _phase2_routes.lock().unwrap();
                shared.append(&mut _route);
            };
        });
    }

    pool.join();
    let phase2_routes = arc_phase2_routes.lock().unwrap();
    let nb_p2 = phase2_routes.len();

    // Drop on empty results
    if phase2_routes.len() == 0 {
        return None;
    }

    // Print the results if verbose
    #[cfg(feature = "verbose")]
    if params.verbose > 0 {
        println!("[P2] Found {} solutions, sorting", phase2_routes.len());
        if params.verbose > 1 {
            for r in phase2_routes.iter() {
                println!(
                    "[P2] {:?} p:{}% q:{}% d:{}",
                    r.actions,
                    r.progression * 100 / r.recipe.progress,
                    r.quality * 100 / r.recipe.quality,
                    r.durability
                );
            }
        }
    }

    // Prune the results for analysis
    // let mut valid_routes : Vec<Craft> = vec![];
    let mut valid_solutions: Vec<SolverResult> = vec![];
    for route in phase2_routes.iter() {
        if route.quality >= route.recipe.quality {
            // valid_routes.push(route.clone());
            valid_solutions.push(SolverResult::from_craft(route)); //nb_p1, nb_p2, true
        }
    }
    // If no craft can make it to 100% HQ, fallback to base results
    if valid_solutions.len() == 0 {
        for route in phase2_routes.iter() {
            valid_solutions.push(SolverResult::from_craft(route));
        }
    }

    Some((valid_solutions, Info::new(nb_p1, nb_p2, true)))
}

/// Load the config from args and make a craft from it
pub fn load_from_config<'a>(
    recipe_name: &str,
    file_name: &str,
    character_name: &str,
) -> (Recipe, Stats) {
    //read craft.toml
    let config: toml::Value = toml::from_str(
        &std::fs::read_to_string(file_name).expect(&format!("Can't open {}", file_name)),
    )
    .unwrap();

    let recp = match config.get(recipe_name) {
        Some(r) => r,
        None => panic!("Can't find value '{}' in '{}'", recipe_name, file_name),
    };

    // Load receipe values
    let recipe = Recipe {
        durability: recp
            .get("durability")
            .expect(&format!(
                "Can't find 'durability' in recipe '{}' on file '{}'",
                recipe_name, file_name
            ))
            .as_integer()
            .expect("Can't convert durability as an integer") as u32,
        progress: recp
            .get("progress")
            .expect(&format!(
                "Can't find 'progress' in recipe '{}' on '{}'",
                recipe_name, file_name
            ))
            .as_integer()
            .expect("Can't convert progress as an integer") as u32,
        quality: recp
            .get("quality")
            .expect(&format!(
                "Can't find 'quality' in recipe '{}' on '{}'",
                recipe_name, file_name
            ))
            .as_integer()
            .expect("Can't convert quality as an integer") as u32,
        progress_divider: recp
            .get("progress_divider")
            .expect(&format!(
                "Can't find 'progress_divider' in recipe '{}' on '{}'",
                recipe_name, file_name
            ))
            .as_integer()
            .expect("Can't convert progress_divider as an integer")
            as u32,
        quality_divider: recp
            .get("quality_divider")
            .expect(&format!(
                "Can't find 'quality_divider' in recipe '{}' on '{}'",
                recipe_name, file_name
            ))
            .as_integer()
            .expect("Can't convert quality_divider as an integer") as u32,
        progress_modifier: recp
            .get("progress_modifier")
            .expect(&format!(
                "Can't find 'progress_modifier' in recipe '{}' on '{}'",
                recipe_name, file_name
            ))
            .as_integer()
            .expect("Can't convert progress_modifier as an integer")
            as u32,
        quality_modifier: recp
            .get("quality_modifier")
            .expect(&format!(
                "Can't find 'quality_modifier' in recipe '{}' on '{}'",
                recipe_name, file_name
            ))
            .as_integer()
            .expect("Can't convert quality_modifier as an integer")
            as u32,
    };

    let cfg = match config.get(character_name) {
        Some(c) => c,
        None => panic!("Can't find '{}' in file '{}'", character_name, file_name),
    };
    let stats = Stats {
        craftsmanship: cfg
            .get("craftsmanship")
            .expect(&format!(
                "Can't find 'craftsmanship' in character '{}' on file '{}'",
                character_name, file_name
            ))
            .as_integer()
            .expect("Can't convert craftsmanship as an integer") as u32,
        control: cfg
            .get("control")
            .expect(&format!(
                "Can't find 'control' in character '{}' on file '{}'",
                character_name, file_name
            ))
            .as_integer()
            .expect("Can't convert control as an integer") as u32,
        max_cp: cfg
            .get("max_cp")
            .expect(&format!(
                "Can't find 'max_cp' in character '{}' on file '{}'",
                character_name, file_name
            ))
            .as_integer()
            .expect("Can't convert max_cp as an integer") as u32,
    };
    (recipe, stats)
}

/// Print all routes in the vect, verbose
pub fn print_routes<'a>(routes: &Option<(Vec<SolverResult>, Info)>) {
    match routes {
        Some((r, _)) => {
            println!("Showing {} routes", r.len());
            for c in r.iter() {
                println!(
                    "\n[{}¤][{}@][{}#] {:?} ",
                    c.quality, c.durability, c.steps, c.actions
                )
            }
        }
        None => println!("No routes to print"),
    }
}

/// Find the route with the least amount of steps
pub fn find_fast_route<'a>(routes: &'a (Vec<SolverResult>, Info)) -> Option<&'a SolverResult> {
    let (_routes, _info) = routes;
    if _routes.len() > 0 {
        _routes.iter().min_by_key(|key| key.steps)
    } else {
        None
    }
}

/// Find the route with the maximum amount of quality
pub fn find_quality_route<'a>(routes: &'a (Vec<SolverResult>, Info)) -> Option<&'a SolverResult> {
    let (_routes, _info) = routes;
    _routes.iter().max_by_key(|key| key.quality)
}

/// Find the route with the maximum of durability left
pub fn find_safe_route(routes: &(Vec<SolverResult>, Info)) -> Option<&SolverResult> {
    let (_routes, _info) = routes;
    if _routes.len() > 0 {
        _routes.iter().max_by_key(|key| key.durability)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_lib_find_route() {
        // Supposed to find a route for the base craft
        let mut craft = crate::craft::Craft::two_star();
        let res = solve_craft(craft.recipe, craft.stats, craft.args);
        assert!(res.unwrap().0.len() > 2000);

        craft.recipe.progress = 0;
        let res = solve_craft(craft.recipe, craft.stats, craft.args);
        println!("{:?}", res);
        assert!(res.is_none());

        craft.recipe.progress = 100;
        craft.stats.max_cp = 0;
        craft.cp = 0;

        let mut res = solve_craft(craft.recipe, craft.stats, craft.args).unwrap();
        println!("{:?}", res);
        assert_eq!(res.0.len(), 1);
        assert_eq!(res.0.pop().unwrap().actions, vec!("basicSynth2"));
    }

    #[test]
    pub fn test_lib_print_route() {
        let craft = crate::craft::Craft::two_star();
        print_routes(&solve_craft(craft.recipe, craft.stats, craft.args));
        print_routes(&None);
    }

    #[test]
    pub fn test_lib_best_solutions() {
        let craft = crate::craft::Craft::two_star();
        let routes = solve_craft(craft.recipe, craft.stats, craft.args).unwrap();
        // assert!(find_fast_route(&(None)).is_none());
        // assert!(find_quality_route(&None).is_none());
        // assert!(find_safe_route(&None).is_none());

        assert!(find_fast_route(&((vec![], Info::default()))).is_none());
        assert!(find_quality_route(&((vec![], Info::default()))).is_none());
        assert!(find_safe_route(&((vec![], Info::default()))).is_none());

        let sol = find_fast_route(&routes).unwrap();
        assert_eq!(sol.steps, 17);
        assert_eq!(sol.quality, 11129);
        assert_eq!(sol.progression, 3750);
        assert_eq!(sol.durability, 0);
        assert_eq!(sol.cp, 7);
        assert_eq!(routes.1.step1_solutions, 9);
        assert_eq!(routes.1.step2_solutions, 21193);
        assert_eq!(routes.1.found_100_percent, true);
        assert_eq!(
            sol.actions,
            vec!(
                "muscleMemory",
                "manipulation",
                "veneration",
                "wasteNot2",
                "groundwork",
                "delicateSynthesis",
                "delicateSynthesis",
                "innovation",
                "preparatoryTouch",
                "preparatoryTouch",
                "preparatoryTouch",
                "preparatoryTouch",
                "innovation",
                "basicTouch",
                "standardTouch",
                "greatStrides",
                "byregotsBlessing",
                "basicSynth2"
            )
        );

        let sol = find_quality_route(&routes).unwrap();
        assert_eq!(sol.steps, 23);
        assert_eq!(sol.quality, 12032);
        assert_eq!(sol.progression, 3675);
        assert_eq!(sol.durability, -5);
        assert_eq!(sol.cp, 3);
        assert_eq!(routes.1.step1_solutions, 9);
        assert_eq!(routes.1.step2_solutions, 21193);
        assert_eq!(routes.1.found_100_percent, true);
        assert_eq!(
            sol.actions,
            vec!(
                "muscleMemory",
                "manipulation",
                "veneration",
                "groundwork",
                "prudentSynthesis",
                "innovation",
                "prudentTouch",
                "basicTouch",
                "standardTouch",
                "advancedTouch",
                "manipulation",
                "innovation",
                "prudentTouch",
                "basicTouch",
                "standardTouch",
                "advancedTouch",
                "innovation",
                "basicTouch",
                "standardTouch",
                "advancedTouch",
                "greatStrides",
                "innovation",
                "byregotsBlessing",
                "basicSynth2"
            )
        );

        let sol = find_safe_route(&routes).unwrap();
        assert_eq!(sol.steps, 22);
        assert_eq!(sol.quality, 11161);
        assert_eq!(sol.progression, 3675);
        assert_eq!(sol.durability, 10);
        assert_eq!(sol.cp, 4);
        assert_eq!(routes.1.step1_solutions, 9);
        assert_eq!(routes.1.step2_solutions, 21193);
        assert_eq!(routes.1.found_100_percent, true);
        assert_eq!(
            sol.actions,
            vec!(
                "muscleMemory",
                "manipulation",
                "veneration",
                "groundwork",
                "carefulSynthesis",
                "prudentTouch",
                "innovation",
                "prudentTouch",
                "prudentTouch",
                "prudentTouch",
                "prudentTouch",
                "manipulation",
                "innovation",
                "prudentTouch",
                "basicTouch",
                "standardTouch",
                "advancedTouch",
                "innovation",
                "basicTouch",
                "standardTouch",
                "greatStrides",
                "byregotsBlessing",
                "basicSynth2"
            )
        );
    }

    #[test]
    pub fn test_load_from_config() {
        load_from_config("default_recipe", "craft.toml", "default_character");
    }

    #[should_panic]
    #[test]
    pub fn test_bad_config_filename() {
        load_from_config("default_recipe", "nocrafto.toml", "default_character");
    }

    #[should_panic]
    #[test]
    pub fn test_bad_config_recipename() {
        load_from_config("bad_recipe", "craft.toml", "default_character");
    }
    #[should_panic]
    #[test]
    pub fn test_bad_config_charname() {
        load_from_config("default_recipe", "craft.toml", "bad_character");
    }
}
