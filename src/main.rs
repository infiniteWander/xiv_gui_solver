use crate::craft::Craft;
use crate::solver::{generate_routes_phase1, generate_routes_phase2};
use crate::specs::{Recipe, Stats};
use clap::Parser;

mod specs;
pub mod action;
pub mod craft;
mod solver;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the receipe
    #[arg(short, long, default_value_t = String::from("default_recipe"))]
    recipe_name: String,

    /// Name of the character
    #[arg(short, long, default_value_t = String::from("default_character"))]
    character_name: String,

    /// The ml file name
    #[arg(short, long, default_value_t = String::from("craft.toml"))]
    file_name: String,
   
    /// The verbose flag
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// The depth of the first pass
    #[arg(short, long, default_value_t = 8)]
    depth: u32,
}

#[derive(Debug)]
struct CustomError(String);

fn main() {
    // Get args
    let args = Args::parse();

    //read craft.toml
    let config: toml::Value = toml::from_str(
        &std::fs::read_to_string(&args.file_name)
        .expect(&format!("Can't open {}",&args.file_name))
        ).unwrap();


    let recp = match config.get(&args.recipe_name){
        Some(r) => r,
        None => panic!("Can't find value '{}' in '{}'",&args.recipe_name, &args.file_name)
    };

    // Load receipe values
    let recipe = Recipe {
        durability: recp
            .get("durability").expect(&format!("Can't find 'durability' in recipe '{}' on file '{}'",
                &args.recipe_name,&args.file_name))
            .as_integer().expect("Can't convert durability as an integer") as u32,
        progress: recp
            .get("progress").expect(&format!("Can't find 'progress' in recipe '{}' on '{}'",
                &args.recipe_name,&args.file_name))
            .as_integer().expect("Can't convert progress as an integer") as u32,
        quality: recp
            .get("quality").expect(&format!("Can't find 'quality' in recipe '{}' on '{}'",
                &args.recipe_name,&args.file_name))
            .as_integer().expect("Can't convert quality as an integer") as u32,
        progress_divider: recp
            .get("progress_divider").expect(&format!("Can't find 'progress_divider' in recipe '{}' on '{}'",
                &args.recipe_name,&args.file_name))
            .as_integer().expect("Can't convert progress_divider as an integer") as u32,
        quality_divider: recp
            .get("quality_divider").expect(&format!("Can't find 'quality_divider' in recipe '{}' on '{}'",
                &args.recipe_name,&args.file_name))
            .as_integer().expect("Can't convert quality_divider as an integer") as u32,
        progress_modifier: recp
            .get("progress_modifier").expect(&format!("Can't find 'progress_modifier' in recipe '{}' on '{}'",
                &args.recipe_name,&args.file_name))
            .as_integer().expect("Can't convert progress_modifier as an integer") as u32,
        quality_modifier: recp
            .get("quality_modifier").expect(&format!("Can't find 'quality_modifier' in recipe '{}' on '{}'",
                &args.recipe_name,&args.file_name))
            .as_integer().expect("Can't convert quality_modifier as an integer") as u32,
    };

    let cfg = match config.get(&args.character_name){
        Some(c) => c,
        None => panic!("Can't find '{}' in file '{}'",&args.character_name,&args.file_name),
    };
    let stats = Stats {
        craftsmanship: cfg
            .get("craftsmanship").expect(&format!("Can't find 'craftsmanship' in character '{}' on file '{}'",
                &args.character_name,&args.file_name))
            .as_integer().expect("Can't convert craftsmanship as an integer") as u32,
        control: cfg
            .get("control").expect(&format!("Can't find 'control' in character '{}' on file '{}'",
                &args.character_name,&args.file_name))
            .as_integer().expect("Can't convert control as an integer") as u32,
        max_cp: cfg
            .get("max_cp").expect(&format!("Can't find 'max_cp' in character '{}' on file '{}'",
                &args.character_name,&args.file_name))
            .as_integer().expect("Can't convert max_cp as an integer") as u32,
    };
    let craft = Craft::new(&recipe, &stats, &args.depth);

    if args.verbose>0{
        println!("Solving...\n");
        println!("[P1] Starting phase 1...");
    }
    let phase1_routes = generate_routes_phase1(craft);
    
    if args.verbose>1{
        println!("[P1] Found {} routes, testing them all...",phase1_routes.len());
        for r in &phase1_routes{
            println!("[P1] {:?} p:{}% q:{}% c:{} d:{}",
                r.actions, 
                r.progression * 100 / r.recipe.progress, 
                r.quality * 100 / r.recipe.quality,
                r.cp,
                r.durability,
                );
            // println!("[P1] {}",r);
        };
    }
    
    let mut phase2_routes = Vec::new();
    for route in phase1_routes {
        if let Some(route) = generate_routes_phase2(route) {
            phase2_routes.push(route);
        }
    }

    if args.verbose>1{
        println!("[P2] Found {} solutions, sorting",phase2_routes.len());
        for r in &phase2_routes{
            println!("[P2] {:?} p:{}% q:{}% d:{}",
                r.actions, 
                r.progression * 100 / r.recipe.progress, 
                r.quality * 100 / r.recipe.quality,
                r.durability);
        };
    }

    let top_route = match phase2_routes.iter().max_by_key(|route| route.quality) {
        Some(top) => top,
        None => return,
    };
    // let top_route = match phase2_routes.iter().min_by_key(|route| route.step_count) {
    //     Some(top) => top,
    //     None => return,
    // };

    let mut content = (&top_route.actions)
        .iter()
        .map(|action| {
            format!("\"{}\"", action.short_name.clone())
        })
        .collect::<Vec<String>>();
    let arg = (top_route.recipe.progress as f32 - top_route.progression as f32) / top_route.get_base_progression() as f32;
    if 0.0 < arg && arg < 1.2 { content.push("\"basicSynth2\"".to_string()); }
    if 1.2 <= arg && arg < 1.8 { content.push("\"carefulSynthesis\"".to_string()); }
    if 1.8 <= arg && arg < 2.0 {
        content.push("\"observe\"".to_string());
        content.push("\"focusedSynthesis\"".to_string());
    }

    if args.verbose>2{
        println!("[F] Top route {:?}",top_route);
    }

    println!("Quality: {}/{}", top_route.quality, top_route.recipe.quality);
    println!("\t[{}]", content.join(", "));
    
    // Wait for enter
    println!();
    println!("Press enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    ()
}
