//! CSIS-616 - References and yaml example
//! 
//! Ralph W. Crosby PhD.
//! 
//! 
//! # Usage
//! 
//!    ```
//!     yaml_dfa filename
//!     ```
//! 
//!    where: `filename` is a yaml file containing the DFA definition
//! 
//! # Output
//! 
//! Debugging outputis produced to `stdout`. Build and run using:
//! 
//! ```
//! cargo run sample.yaml
//! ```

use std::io::Write;
use serde::{Deserialize};

/// # Deterministic Finite Automata Structure
/// 
/// Create a structure that the YAML files will be deserialized into.
/// Note the use of the `Deserialize` trait
/// 
#[derive(Debug, Deserialize)]
struct DFA {

    alphabet: Vec<char>,
    start: u32,
    accept: Vec<u32>,
    transitions: Vec<Vec<u32>>,
    
    // This field isn't loaded from the YAML file so we need
    // to provide a default value for it
    #[serde(default)]
    n_states: usize

}

fn main() {

    let filename = get_filename(std::env::args());

    // Load the yaml file getting a Box pointing to a DFA
    // instance on the heap
    let mut d = DFA::new_from_file(&filename);

    d.print("Before");

    // Figure out how many states are in the transition table
    d.compute_states();

    d.print("After");

}

/// Return the filename passed as the first parameter
fn get_filename(args: std::env::Args) -> String {

    // Get the arguments as a vector
    let args: Vec<String> = args.collect();

    // Make sure only one argument was passed
    if args.len() != 2 {
        writeln!(std::io::stderr(), "Usage: hw1 dfafile")
            .unwrap();
        std::process::exit(1);
    }
    
    args[1].to_string()
    
}

/*

Non-working version, can't return a reference to something on the stack

fn get_filename(args: std::env::Args) -> &str {

    // Get the arguments as a vector
    let args: Vec<String> = args.collect();

    // Make sure only one argument was passed
    if args.len() != 2 {
        writeln!(std::io::stderr(), "Usage: hw1 dfafile")
            .unwrap();
        std::process::exit(1);
    }
    
    Show this doesn't work
    &args[1]
}
*/    

impl DFA {

    /// Create and return a DFA on the heap
    /// 
    /// Load the .yaml file specified into a DFA structure
    /// on the heap and return a point to it via a Box.

    fn new_from_file(filename: &str) -> Box<DFA> {

        let f = std::fs::File::open(filename)
                    .expect("Unable to open input");

        // Deserialize into the heap and return the pointer
        Box::new(serde_yaml::from_reader(f)
                    .expect("Unable to parse yaml") )

    }

    /*

    This version returns an actual DFA object on the stack.
    Not efficient; all we really want is to return a pointer
    to an object on the heap. 

    fn new_from_file(filename: &str) -> DFA {

        let f = std::fs::File::open(filename)
                    .expect("Unable to open input");

        let d = serde_yaml::from_reader(f)
                    .expect("Unable to parse yaml");

        d

    }
        
    */


    /// Compute the number of states
    fn compute_states(&mut self) {
        self.n_states = self.transitions.len();
    }

    /// Guess what this does!
    fn print(&self, s: &str) {
        println!("{}: {:?}", s, self);
    }

}

