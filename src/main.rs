use crate::craft::Craft;
use crate::solver::{generate_routes_phase1, generate_routes_phase2};
use crate::specs::{Recipe, Stats};

mod specs;
pub mod action;
pub mod craft;
mod solver;

fn main() {
    //read craft.toml
    let config: toml::Value = toml::from_str(&std::fs::read_to_string("craft.toml").unwrap()).unwrap();
    let recipe = Recipe {
        durability: config["recipe"]["durability"].as_integer().unwrap() as u32,
        progress: config["recipe"]["progress"].as_integer().unwrap() as u32,
        quality: config["recipe"]["quality"].as_integer().unwrap() as u32,
        progress_divider: config["recipe"]["progress_divider"].as_integer().unwrap() as u32,
        quality_divider: config["recipe"]["quality_divider"].as_integer().unwrap() as u32,
        progress_modifier: config["recipe"]["progress_modifier"].as_integer().unwrap() as u32,
        quality_modifier: config["recipe"]["quality_modifier"].as_integer().unwrap() as u32,
    };
    let stats = Stats {
        craftsmanship: config["stats"]["craftsmanship"].as_integer().unwrap() as u32,
        control: config["stats"]["control"].as_integer().unwrap() as u32,
        max_cp: config["stats"]["max_cp"].as_integer().unwrap() as u32,
    };
    let craft = Craft::new(&recipe, &stats);
    println!("Solving...");
    println!();
    let phase1_routes = generate_routes_phase1(craft);
    let mut phase2_routes = Vec::new();
    for route in phase1_routes {
        if let Some(route) = generate_routes_phase2(route) {
            phase2_routes.push(route);
        }
    }
    let top_route = phase2_routes.iter().max_by_key(|route| route.quality).unwrap();
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
    println!("Quality: {}/{}", top_route.quality, top_route.recipe.quality);
    println!("\t[{}]", content.join(", "));
    // wait for enter
    println!();
    println!("Press enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    ()
}
