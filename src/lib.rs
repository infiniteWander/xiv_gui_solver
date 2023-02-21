use crate::io::SolverResult;
use crate::{
    craft::Craft,
    specs::{Recipe,Stats},
    io::Parameters,
};
use threadpool::ThreadPool;
use std::sync::{Arc,Mutex};

mod solver;
mod specs;
pub mod action;
pub mod craft;
pub mod io;



/// Create stats

/// Create recipe

/// Create parameters


/// Create from config



/// Solve the craft with given arguments, this functions calls threads and must own it's values
pub fn solve_craft<'a>(recipe: Recipe, stats: Stats, params: Parameters) -> Option<Vec<SolverResult>>{
    // Load the craft with given arguments
    let craft = Craft::new(recipe,stats,params);

    // Start a threadpool
    let pool = ThreadPool::new(params.threads);

    #[cfg(feature = "verbose")]
    if params.verbose>0{
        println!("Solving...\n");
        println!("[P1] Starting phase 1...");
    }

    let phase1_routes = solver::generate_routes_phase1(craft);
    let nb_p1 = phase1_routes.len();

    #[cfg(feature = "verbose")]
    if params.verbose>0{
        println!("[P1] Found {} routes, testing them all...",phase1_routes.len());
        if params.verbose>1{ for r in &phase1_routes{
            println!("[P1] {:?} p:{}% q:{}% c:{} d:{}",r.actions,r.quality * 100 / r.recipe.quality,r.cp,r.durability);
        }}
    }

    // Core algorithm, fill all found routes with the best route (doesn't branch, just replace)
    let arc_phase2_routes = Arc::new(Mutex::new(Vec::<Craft>::new()));

    for route in phase1_routes {
        let _phase2_routes = Arc::clone(&arc_phase2_routes);

        pool.execute(move || {
            if let Some(_route) = solver::generate_routes_phase2(route){
                let mut shared = _phase2_routes.lock().unwrap();
                shared.push(_route);
            };
        });
    }

    pool.join();
    let phase2_routes = arc_phase2_routes.lock().unwrap();
    let nb_p2 = phase2_routes.len();

    // Print the results if verbose
    #[cfg(feature = "verbose")]
    if params.verbose>0{
        println!("[P2] Found {} solutions, sorting",phase2_routes.len());
        if params.verbose>1{ for r in phase2_routes.iter(){
                println!("[P2] {:?} p:{}% q:{}% d:{}", r.actions, r.progression * 100 / r.recipe.progress, r.quality * 100 / r.recipe.quality, r.durability);
        }}
    }

    // Prune the results for analysis
    let mut valid_routes : Vec<Craft> = vec![];
    let mut valid_solutions: Vec<SolverResult> = vec![];
    for route in phase2_routes.iter(){
        if route.quality>=route.recipe.quality{
            valid_routes.push(route.clone());
            valid_solutions.push(SolverResult::from_craft(route,nb_p1,nb_p2,true));
        }
    }
    // If no craft can make it to 100% HQ, fallback to base results
    if valid_solutions.len()==0{
        for route in phase2_routes.iter(){valid_solutions.push(SolverResult::from_craft(route,nb_p1,nb_p2,false));}
    }

    Some(valid_solutions)
}

/// Load the config from args and make a craft from it
pub fn load_from_config<'a>(recipe_name: &str, file_name: &str, character_name: &str) -> (Recipe,Stats) {
    //read craft.toml
    let config: toml::Value = toml::from_str(
        &std::fs::read_to_string(file_name)
        .expect(&format!("Can't open {}",file_name))
        ).unwrap();

    let recp = match config.get(recipe_name){
        Some(r) => r,
        None => panic!("Can't find value '{}' in '{}'",recipe_name, file_name)
    };

    // Load receipe values
    let recipe = Recipe {
        durability: recp
            .get("durability").expect(&format!("Can't find 'durability' in recipe '{}' on file '{}'",
                recipe_name,file_name))
            .as_integer().expect("Can't convert durability as an integer") as u32,
        progress: recp
            .get("progress").expect(&format!("Can't find 'progress' in recipe '{}' on '{}'",
                recipe_name,file_name))
            .as_integer().expect("Can't convert progress as an integer") as u32,
        quality: recp
            .get("quality").expect(&format!("Can't find 'quality' in recipe '{}' on '{}'",
                recipe_name,file_name))
            .as_integer().expect("Can't convert quality as an integer") as u32,
        progress_divider: recp
            .get("progress_divider").expect(&format!("Can't find 'progress_divider' in recipe '{}' on '{}'",
                recipe_name,file_name))
            .as_integer().expect("Can't convert progress_divider as an integer") as u32,
        quality_divider: recp
            .get("quality_divider").expect(&format!("Can't find 'quality_divider' in recipe '{}' on '{}'",
                recipe_name,file_name))
            .as_integer().expect("Can't convert quality_divider as an integer") as u32,
        progress_modifier: recp
            .get("progress_modifier").expect(&format!("Can't find 'progress_modifier' in recipe '{}' on '{}'",
                recipe_name,file_name))
            .as_integer().expect("Can't convert progress_modifier as an integer") as u32,
        quality_modifier: recp
            .get("quality_modifier").expect(&format!("Can't find 'quality_modifier' in recipe '{}' on '{}'",
                recipe_name,file_name))
            .as_integer().expect("Can't convert quality_modifier as an integer") as u32,
    };

    let cfg = match config.get(character_name){
        Some(c) => c,
        None => panic!("Can't find '{}' in file '{}'",character_name,file_name),
    };
    let stats = Stats {
        craftsmanship: cfg
            .get("craftsmanship").expect(&format!("Can't find 'craftsmanship' in character '{}' on file '{}'",
                character_name,file_name))
            .as_integer().expect("Can't convert craftsmanship as an integer") as u32,
        control: cfg
            .get("control").expect(&format!("Can't find 'control' in character '{}' on file '{}'",
                character_name,file_name))
            .as_integer().expect("Can't convert control as an integer") as u32,
        max_cp: cfg
            .get("max_cp").expect(&format!("Can't find 'max_cp' in character '{}' on file '{}'",
                character_name,file_name))
            .as_integer().expect("Can't convert max_cp as an integer") as u32,
    };
    (recipe,stats)
}

/// Print all routes in the vect, verbose
pub fn print_routes<'a>(routes: Option<Vec<SolverResult>>){
    match routes{
        Some(r) => {
            println!("Showing {} routes",r.len());
            for c in r.iter(){
                println!("\n[{}¤][{}@][{}#] {:?} ",c.quality,c.durability,c.steps,c.actions)                
            }
        },
        None => println!("No routes to print")
    }
}

// TODO: Fix it for not completed crafts
/// Find the route with the least amount of steps
pub fn find_fast_route(routes: &Option<Vec<SolverResult>>) -> Option<&SolverResult>{
    let res = match routes {
         Some(route) => route.iter().min_by_key(|key| key.steps),
         None => None,
    };
    res
}

/// Find the route with the maximum amount of quality
pub fn find_quality_route(routes: &Option<Vec<SolverResult>>) -> Option<&SolverResult>{
    match routes {
         Some(route) => route.iter().max_by_key(|key| key.quality),
         None => None,
    }
}

/// Find the route with the maximum of durability left
pub fn find_safe_route(routes: &Option<Vec<SolverResult>>) -> Option<&SolverResult>{
    match routes {
         Some(route) => route.iter().min_by_key(|key| key.durability),
         None => None,
    }
}