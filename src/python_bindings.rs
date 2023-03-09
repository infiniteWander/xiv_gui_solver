//! The binding module
//! It contains the python bindings, and the structs
//! needed to hold all parameters needed at runtime
//!
//! Making the bindings a build feature is a WIP

use crate::Craft;
#[cfg(not(feature = "no_python"))]
use crate::Parameters;
use core::fmt::Display;
use pyo3::prelude::*;

/// A final stripped down version of a craft
/// used for final print and talking with python
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
    // /// The amount of solutions found during the first step of the solving (Used for debug) (Deprecated)
    // #[pyo3(get)]
    // pub step1_solutions: usize,
    // /// The amount of solutions found at the end of the second step of solving (Used for debug) (Deprecated)
    // #[pyo3(get)]
    // pub step2_solutions: usize,
    // #[pyo3(get)]
    // /// Wether a 100% HQ solution was found (Deprecated)
    // pub found_100_percent: bool,
}

/// Global info on the execution
#[derive(Debug)]
#[pyclass]
pub struct Info {
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

/// Solution information, contains information about the execution
impl Info {
    /// Make a new information struct
    pub fn new(
        step1_solutions: usize,
        step2_solutions: usize,
        found_100_percent: bool,
        // params: &'Parameters
    ) -> Self {
        Self {
            step1_solutions,
            step2_solutions,
            found_100_percent,
        }
    }

    /// Default value of the info, only used for debug purposes
    pub fn default() -> Self {
        Self {
            step1_solutions: 0,
            step2_solutions: 0,
            found_100_percent: false,
        }
    }
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
    pub fn from_craft(craft: &Craft) -> SolverResult {
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

////// Python Bindings //////

/// A Python module implemented in Rust.
#[cfg(not(feature = "no_python"))]
#[pymodule]
fn xiv_csolver_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test_result, m)?)?;
    m.add_function(wrap_pyfunction!(test_final_results, m)?)?;
    m.add_function(wrap_pyfunction!(test_info, m)?)?;
    m.add_function(wrap_pyfunction!(solve_from_python, m)?)?;
    Ok(())
}

/// A test result, useful for debugging, all values are set to zero as per the
/// [`default`](crate::python_bindings::SolverResult::default())
/// ```
/// # use xiv_csolver_lib::python_bindings::SolverResult;
/// SolverResult {
///     steps: 0,
///     progression: 0,
///     quality: 0,
///     durability: 0,
///     total_progression: 0,
///     total_quality: 0,
///     total_durability: 0,
///     actions: vec!["Act1".to_string(), "Act2".to_string()],
///     cp: 0,
///     total_cp: 0,
/// };
/// ```
#[cfg(not(feature = "no_python"))]
#[pyfunction]
pub fn test_result() -> SolverResult {
    SolverResult::default()
}

/// A test info, useful for debugging, all values are set to zero as per the
/// [`default`](crate::python_bindings::Info::default())
/// ```
/// # use xiv_csolver_lib::python_bindings::Info;
/// Info {
///     step1_solutions: 0,
///     step2_solutions: 0,
///     found_100_percent: false,
/// };
/// ```
#[cfg(not(feature = "no_python"))]
#[pyfunction]
pub fn test_info() -> Info {
    Info::default()
}

/// A test result array, useful for debugging, all values are set to zero as per the
/// [`default`](crate::python_bindings::Info::default())
/// ```
/// # use xiv_csolver_lib::python_bindings::{Info};
/// # use xiv_csolver_lib::python_bindings::SolverResult;
/// (
///     vec![SolverResult::default(), SolverResult::default()],
///     Info::default(),
/// );
/// ```
#[cfg(not(feature = "no_python"))]
#[pyfunction]
pub fn test_final_results() -> (Vec<SolverResult>, Info) {
    (
        vec![SolverResult::default(), SolverResult::default()],
        Info::default(),
    )
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
pub fn solve_from_python(
    py: Python<'_>,
    values: &PyAny,
) -> PyResult<Option<(Vec<SolverResult>, Info)>> {
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
mod test {
    use crate::python_bindings::{test_final_results, test_info, test_result};

    #[test]
    pub fn test_function_call() {
        test_final_results();
        test_info();
        test_result();
    }
}
