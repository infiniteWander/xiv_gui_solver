//! The main IO module
//! For now it contains the python bindings, the Args bindings (for clap) and the struct
//! needed to hold all parameters needed at runtime
//!
//! Moving the bindings and making them non mandatory is a WIP

use crate::Craft;
use clap::Parser;
use core::fmt::Display;

// #[cfg(not(feature = "no_python"))]
use pyo3::prelude::*;

/// The parameters needed at runtime, used since Args cannot easily be passed to python
#[derive(Debug, Clone, Copy)]
pub struct Parameters {
    /// The number of threads to use
    /// TODO: Add an auto feature to guess threads depending on architecture and action pool
    pub threads: usize,
    /// The level of verbosity from 0 to 3
    pub verbose: u8,
    /// How many steps are allowed for finishing the craft
    /// TODO: Be a little smarted about this approach and stop guessing if enough starters are found
    /// and continue guessing if more are needed
    pub depth: u32,
    /// Wether to try every mesure to finish the craft or not
    pub desperate: bool,
    /// How early byregot will be used, setting it too low high miss easy solutions (for small crafts)
    /// but setting it too low will have the solver try byregot way too often adding many useless crafts
    /// in the pool of solutions
    pub byregot_step: u8,
}

/// The args passed to the CLI, used by clap to generate a nice argument parser
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

    /// Thread counts, default is 4
    #[arg(short, long, default_value_t = 4)]
    pub threads: usize,

    /// Desperate mode, will try to finish the craft above all
    #[arg(short = 'D', long, default_value_t = false)]
    pub desperate: bool,

    /// Long mode, will try to find more solutions, at the expense of time
    #[arg(short = 'l', long, default_value_t = false)]
    pub long: bool,
}

/// A final stripped down version of a craft
/// used for final print and talking with python
/// TODO: Half of these parameter are unique and SHOULD NOT be replicated on every solution
#[derive(Debug)]
#[pyclass]
pub struct SolverResult {
    /// The number of steps this particular craft takes
    #[pyo3(get)]
    pub steps: u32,
    /// The quality obtained at the end of the craft (absolute value)
    #[pyo3(get)]
    pub quality: u32,
    /// The amount of progression obtained at the end of the craft (absolute value)
    #[pyo3(get)]
    pub progression: u32,
    /// The remaining durability at the end of this craft
    #[pyo3(get)]
    pub durability: i32,
    /// The remaining crafting points at the end of this craft
    #[pyo3(get)]
    pub cp: i32,
    /// The total amount of crafting points the crafter had available (Deprecated)
    #[pyo3(get)]
    pub total_cp: u32,
    /// The total amount of progression needed to finish the craft (Deprecated)
    #[pyo3(get)]
    pub total_progression: u32,
    /// The total quality needed to make this craft 100% HQ (Deprecated)
    #[pyo3(get)]
    pub total_quality: u32,
    /// The total durability for this craft (Deprecated)
    #[pyo3(get)]
    pub total_durability: u32,
    /// The list of actions, as a string
    #[pyo3(get)]
    pub actions: Vec<String>,
    /// The amount of solutions found during the first step of the solving (Used for debug) (Deprecated)
    #[pyo3(get)]
    pub step1_solutions: usize,
    /// The amount of solutions found at the end of the second step of solving (Used for debug) (Deprecated)
    #[pyo3(get)]
    pub step2_solutions: usize,
    #[pyo3(get)]
    /// Wether a 100% HQ solution was found (Deprecated)
    pub found_100_percent: bool,
}

/// Pretty display for SolverResult
impl Display for SolverResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.actions)
    }
}

impl SolverResult {
    /// Create a SolverResult from a solution
    /// Using this returns a solution, usable by Python
    pub fn from_craft(
        craft: &Craft,
        step1_solutions: usize,
        step2_solutions: usize,
        found_100_percent: bool,
    ) -> SolverResult {
        let mut actions = craft
            .actions
            .iter()
            .map(|action| format!("{}", action.short_name))
            .collect::<Vec<String>>();
        // Todo: Stop adding these by hand
        let arg = (craft.recipe.progress as f32 - craft.progression as f32)
            / craft.get_base_progression() as f32;
        if 0.0 < arg && arg < 1.2 {
            actions.push("basicSynth2".to_string());
        }
        if 1.2 <= arg && arg < 1.8 {
            actions.push("carefulSynthesis".to_string());
        }
        if 1.8 <= arg && arg < 2.0 {
            actions.push("observe".to_string());
            actions.push("focusedSynthesis".to_string());
        }
        SolverResult {
            steps: craft.step_count,
            progression: craft.progression,
            quality: craft.quality,
            durability: craft.durability,
            actions: actions,
            step1_solutions,
            step2_solutions,
            found_100_percent,
            total_progression: craft.recipe.progress,
            total_quality: craft.recipe.quality,
            total_durability: craft.recipe.durability,
            cp: craft.cp,
            total_cp: craft.stats.max_cp,
        }
    }
}

#[pymethods]
impl SolverResult {
    /// Default method to create an empty solution, only used for debug purposes
    /// to test wether the import worked
    /// ```py
    /// import xiv_csolver_lib
    /// print(xiv_csolver_lib.default())
    /// ```
    #[staticmethod]
    pub fn default() -> Self {
        Self {
            steps: 0,
            progression: 0,
            quality: 0,
            durability: 0,
            total_progression: 0,
            total_quality: 0,
            total_durability: 0,
            actions: vec!["Act1".to_string(), "Act2".to_string()],
            step1_solutions: 0,
            step2_solutions: 0,
            found_100_percent: false,
            cp: 0,
            total_cp: 0,
        }
    }

    /// Implement a pretty_print method
    /// Mainly used for debug purposes
    pub fn pretty_print(&self) {
        println!(
            "Quality: [{}/{}] | Durability: [{}/{}] | Cp : [{}/{}] | Steps : {}",
            self.quality,
            self.total_quality,
            self.durability,
            self.total_durability,
            self.cp,
            self.total_cp,
            self.steps
        );
        println!("{:?}", self.actions);
    }
}

impl Parameters {
    /// Create parameters from the arguments parsed by clap
    pub fn from_args(args: &Args) -> Self {
        Self {
            depth: if args.desperate { 11 } else { 8 },
            threads: args.threads,
            verbose: args.verbose,
            desperate: args.desperate,
            byregot_step: if args.long { 6 } else { 8 },
        }
    }
}

////// Python Bindings //////

/// A Python module implemented in Rust.
#[cfg(not(feature = "no_python"))]
#[pymodule]
fn xiv_csolver_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test_result, m)?)?;
    m.add_function(wrap_pyfunction!(solve_from_python, m)?)?;
    Ok(())
}

/// A test result, useful for debugging, all values are set to zero as per the
/// [`default`](crate::io::SolverResult::default)
/// ```
/// # use xiv_csolver_lib::io::SolverResult;
/// SolverResult{
///     steps: 0,
///     progression: 0,
///     quality: 0,
///     durability: 0,
///         total_progression: 0,
///         total_quality: 0,
///         total_durability: 0,
///         actions: vec!["Act1".to_string(), "Act2".to_string()],
///         step1_solutions: 0,
///         step2_solutions: 0,
///         found_100_percent: false,
///         cp: 0,
///         total_cp: 0,
///     };
/// ```
#[cfg(not(feature = "no_python"))]
#[pyfunction]
pub fn test_result() -> SolverResult {
    SolverResult::default()
}

/// Create a stat struct stats with the base values
#[cfg(not(feature = "no_python"))]
use crate::{solve_craft, Recipe, Stats};

/// Solve a craft from parameters given by python
/// The values are all obtained by a `&PyAny`, wich must guaranty that several attributes are
/// accesible by `__getattr__`
///
/// ```python
/// import xiv_craft_solver, rich
///
/// class FeedingClass:
///     durability = 70
///     progress = 3900
///     quality = 10920
///     progress_divider = 130
///     quality_divider = 115
///     progress_modifier = 80
///     quality_modifier = 70
///     craftsmanship = 4041
///     control = 3959
///     max_cp = 602
///     # Config
///     depth = 10
///     byregot_step = 10
///     desperate = False
///     threads = 8
///     verbose = 0
/// crafting_feeder = FeedingClass()
/// rich.inspect(xiv_csolver_lib.solve_from_python(crafting_feeder))
/// ```

#[cfg(not(feature = "no_python"))]
#[pyfunction]
pub fn solve_from_python(py: Python<'_>, values: &PyAny) -> PyResult<Option<Vec<SolverResult>>> {
    // Create Recipe
    let recipe = Recipe {
        durability: values.getattr("durability")?.extract()?,
        progress: values.getattr("progress")?.extract()?,
        progress_divider: values.getattr("progress_divider")?.extract()?,
        progress_modifier: values.getattr("progress_modifier")?.extract()?,
        quality: values.getattr("quality")?.extract()?,
        quality_divider: values.getattr("quality_divider")?.extract()?,
        quality_modifier: values.getattr("quality_modifier")?.extract()?,
    };

    // Create Stats
    let stats = Stats {
        craftsmanship: values.getattr("craftsmanship")?.extract()?,
        control: values.getattr("control")?.extract()?,
        max_cp: values.getattr("max_cp")?.extract()?,
    };

    // Create parameters
    let param = Parameters {
        depth: values.getattr("depth")?.extract()?,
        byregot_step: values.getattr("byregot_step")?.extract()?,
        desperate: values.getattr("desperate")?.extract()?,
        threads: values.getattr("threads")?.extract()?,
        verbose: values.getattr("verbose")?.extract()?,
    };

    // println!("{:?} len: {:?} ",values,values.getattr("len()"));
    py.allow_threads(move || {
        let res = solve_craft(recipe, stats, param);
        Ok(res)
    })
}

#[cfg(test)]
mod tests {
    use crate::io::Args;
    use crate::{Parameters, SolverResult};
    use clap::Parser;
    use pretty_assertions::{assert_eq, assert_ne};

    impl PartialEq for Parameters {
        fn eq(&self, rhs: &Parameters) -> bool {
            (self.byregot_step == rhs.byregot_step)
                & (self.depth == rhs.depth)
                & (self.desperate == rhs.desperate)
                & (self.threads == rhs.threads)
        }
    }

    #[test]
    pub fn test_parameters_derived_attributes() {
        let def = Parameters {
            byregot_step: 0,
            depth: 0,
            desperate: false,
            threads: 42,
            verbose: 1,
        };
        let def2 = def;
        let def3 = def2.clone();

        assert_eq!(def2, def3);
    }

    #[test]
    pub fn test_display() {
        let def = SolverResult::default();
        assert_ne!(def.to_string(), "");
        assert_eq!(def.to_string(), "[\"Act1\", \"Act2\"]");
        def.pretty_print();
    }

    #[test]
    pub fn test_debug() {
        let def = SolverResult::default();
        assert_eq!(format!("{:?}", def),"SolverResult { steps: 0, quality: 0, progression: 0, durability: 0, cp: 0, total_cp: 0, total_progression: 0, total_quality: 0, total_durability: 0, actions: [\"Act1\", \"Act2\"], step1_solutions: 0, step2_solutions: 0, found_100_percent: false }");
    }

    #[test]
    pub fn test_arguments() {
        // Create false argument parameter
        let a = Args {
            character_name: "Bob GZ".to_string(),
            depth: 77,
            desperate: false,
            file_name: "test.toml".to_string(),
            long: false,
            recipe_name: "Belladone".to_string(),
            threads: 11,
            verbose: 1,
        };
        let a2 = a.clone();
        println!("{:?}", a2);
        // Test Custom recipe
        let a3 = Args::parse_from([
            "ultraman",
            "-r",
            "recipe",
            "-c",
            "dan",
            "-f",
            "bigfile.toml",
            "-d",
            "45",
            "-t",
            "4",
            "-D",
            "-l",
            "-vvv",
        ]);
        let _ = Args::parse_from(["-V"]);
        let _ = Args::parse_from(["-H"]);
        assert_eq!(a3.recipe_name, "recipe");
        assert_eq!(a3.character_name, "dan");
        assert_eq!(a3.file_name, "bigfile.toml");
        assert_eq!(a3.verbose, 3);
        assert_eq!(a3.depth, 45);
        assert_eq!(a3.threads, 4);
        assert_eq!(a3.desperate, true);
        assert_eq!(a3.long, true);

        // Test default parameters
        let a4 = Args::parse_from(["bobGZ"]);
        assert_eq!(a4.recipe_name, "default_recipe");
        assert_eq!(a4.character_name, "default_character");
        assert_eq!(a4.file_name, "craft.toml");
        assert_eq!(a4.verbose, 0);
        assert_eq!(a4.depth, 8);
        assert_eq!(a4.threads, 4);
        assert_eq!(a4.desperate, false);
        assert_eq!(a4.long, false);
    }

    #[test]
    pub fn test_parameters() {}
}
