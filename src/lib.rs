use crate::{
    craft::Craft,
    specs::{Recipe,Stats},
};
use std::time::{Instant};
use threadpool::ThreadPool;
use std::sync::{Arc,Mutex};
use clap::Parser;
use pyo3::prelude::*;

mod solver;
mod specs;
pub mod action;
pub mod craft;


#[derive(Debug, Clone, Copy)]
pub struct Parameters {
    pub threads: usize,
    pub verbose: u8,
    pub depth: u32,
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the receipe
    #[arg(short, long, default_value_t = String::from("default_recipe"))]
    pub recipe_name: String,

    /// Name of the character
    #[arg(short, long, default_value_t = String::from("default_character"))]
    pub character_name: String,

    /// The ml file name
    #[arg(short, long, default_value_t = String::from("craft.toml"))]
    pub file_name: String,
   
    /// The verbose flag
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// The depth of the first pass
    #[arg(short, long, default_value_t = 8)]
    pub depth: u32,

    /// Thread counts, default is 4 (can you even run ff with less ?)
    #[arg(short, long, default_value_t = 4)]
    pub threads: usize,
}

#[pyfunction]
pub fn pouet()-> &'static str {
    return "pouet";
}

/// A Python module implemented in Rust.
#[pymodule]
fn xiv_craft_solver_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pouet, m)?)?;
    Ok(())
}

/// Solve the craft with given arguments
pub fn solve_craft<'a>(recipe: Recipe, stats: Stats, params: Parameters) -> Option<Vec<Craft<'a>>>{
    // Load the craft with given arguments
    let craft = Craft::new(recipe,stats,params);
    // Start timer
    let now = Instant::now();

    // Start a threadpool
    let pool = ThreadPool::new(params.threads);

    if params.verbose>0{
        println!("Solving...\n");
        println!("[P1] Starting phase 1...");
    }
    let phase1_routes = solver::generate_routes_phase1(craft);
    
    if params.verbose>0{
        println!("[P1] Found {} routes, testing them all...",phase1_routes.len());
        if params.verbose>1{
            for r in &phase1_routes{
                println!("[P1] {:?} p:{}% q:{}% c:{} d:{}",
                    r.actions, 
                    r.progression * 100 / r.recipe.progress, 
                    r.quality * 100 / r.recipe.quality,
                    r.cp,
                    r.durability,
                    );
            };
        }
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
    
    // Print the results if verbose
    if params.verbose>0{
        println!("[P2] Found {} solutions, sorting",phase2_routes.len());

        if params.verbose>1{
            for r in phase2_routes.iter(){
                println!("[P2] {:?} p:{}% q:{}% d:{}",
                    r.actions, 
                    r.progression * 100 / r.recipe.progress, 
                    r.quality * 100 / r.recipe.quality,
                    r.durability);
            };
        }
    }

    // Copy the valid results for analysis, by default only the valid one are copied
    let mut valid_routes : Vec<Craft> = vec![];
    for route in phase2_routes.iter(){
        if route.quality>=route.recipe.quality{
            valid_routes.push(route.clone());
        }
    }

    if valid_routes.len()==0{
        for route in phase2_routes.iter(){valid_routes.push(route.clone())} // Deep copy through the mutex guard
    }


    // Select best route TODO: Seperate function
    let top_route = match valid_routes.iter().max_by_key(|route| route.quality) {
        Some(top) => top,
        None => {
            println!("[P2] No route could finish the craft.\n[P2] Runtime {}ms. Now exiting...",now.elapsed().as_millis());
            return None;
        },
    };

    // Print best route TODO: Seperate function
    let mut content = (&top_route.actions)
        .iter()
        .map(|action| {
            format!("\"{}\"", action.short_name.clone())
        })
        .collect::<Vec<String>>();

    // Setting something to print, adding the missing actions TODO: Change this behaviour and move to separate function
    let arg = (top_route.recipe.progress as f32 - top_route.progression as f32) / top_route.get_base_progression() as f32;
    if 0.0 < arg && arg < 1.2 { content.push("\"basicSynth2\"".to_string()); }
    if 1.2 <= arg && arg < 1.8 { content.push("\"carefulSynthesis\"".to_string()); }
    if 1.8 <= arg && arg < 2.0 {
        content.push("\"observe\"".to_string());
        content.push("\"focusedSynthesis\"".to_string());
    }

    if params.verbose>2{
        println!("[F] Top route {:?}",top_route);
    }

    println!("Quality: {}/{}", top_route.quality, top_route.recipe.quality);
    println!("\t[{}]", content.join(", "));
    
    // Wait for enter TODO: Remove
    println!();
    println!("Program finished sucessfuly in {}ms and found {} solutions, [prog:{}]",
        now.elapsed().as_millis(),
        phase2_routes.len(),
        top_route.recipe.progress);
    println!("Press enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    Some(valid_routes)
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

// fn make_craft_from_values(){

// }