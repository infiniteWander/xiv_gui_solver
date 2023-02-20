use crate::Craft;
use core::fmt::Display;
use clap::Parser;

#[cfg(not(feature="no_python"))]
use pyo3::prelude::*;

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


/// A final stripped down version of a craft
/// used for final print and talking with python
#[derive(Debug)]
#[pyclass]
pub struct SolverResult{
    #[pyo3(get)]
    pub steps: u32,
    #[pyo3(get)]
    pub quality: u32,
    #[pyo3(get)]
    pub progression: u32,
    #[pyo3(get)]
    pub durability: i32,
    #[pyo3(get)]
    pub actions: Vec<String>,
    #[pyo3(get)]
    pub step1_solutions: u32,
    #[pyo3(get)]
    pub step2_solutions: u32,
}

#[pyfunction]
pub fn pouet()-> SolverResult {
    // return "pouet";
    SolverResult{
        steps:0,
        progression: 0,
        quality:0,
        durability:0,
        actions:vec!["BLLLL".to_string(),"WWWWW".to_string()],
        step1_solutions:0,
        step2_solutions:0,

    }
}

/// Pretty display for SolverResult
impl Display for SolverResult{
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        println!("{:?}",self.actions);
        Ok(())
    }
}


impl SolverResult{
    pub fn from_craft(&self,craft: & Craft)->SolverResult{
        // Todo: recreate actions
        // Where steps ?
        SolverResult{
            steps:craft.step_count,
            progression: craft.progression,
            quality:craft.quality,
            durability:craft.durability,
            actions:vec!["BLLLL".to_string(),"WWWWW".to_string()],
            step1_solutions:0,
            step2_solutions:0,
        }

    }

}


/// Python Bindings

/// A Python module implemented in Rust.
#[cfg(not(feature="no_python"))]
#[pymodule]
fn xiv_craft_solver(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pouet, m)?)?;
    Ok(())
}

pub fn e(){

}