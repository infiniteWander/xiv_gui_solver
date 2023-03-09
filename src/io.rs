//! The main IO module
//! For now it contains the Args bindings (for clap) and the struct
//! needed to hold all parameters needed at runtime

use clap::Parser;

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

#[cfg(test)]
mod tests {
    use crate::io::{Args, Parameters};
    use crate::python_bindings::SolverResult;
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
        let a3 = crate::io::Args::parse_from([
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
    pub fn test_parameters() {
        // Test a random config
        let ar = Args::parse_from([
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
        let p = Parameters::from_args(&ar);
        assert_eq!(p.depth, 11);
        assert_eq!(p.verbose, 3);
        assert_eq!(p.threads, 4);
        assert_eq!(p.desperate, true);
        assert_eq!(p.byregot_step, 6);

        // Test defaults
        let ar = Args::parse_from(&["a"]);
        let p = Parameters::from_args(&ar);
        assert_eq!(p.depth, 8);
        assert_eq!(p.verbose, 0);
        assert_eq!(p.threads, 4);
        assert_eq!(p.desperate, false);
        assert_eq!(p.byregot_step, 8);
    }
}
