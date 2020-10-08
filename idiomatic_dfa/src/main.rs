//! CSIS-616 - Program #2 - Idomatic
//! 
//! Ralph W. Crosby PhD.
//! 
//! Process a yaml format deterministic finite automaton producing
//! - A textual representation of the internal state graph
//! - A Graphviz `.dot` file representing the graph
//! 
//! # Usage
//! 
//! ```
//! idomatic_dfa filename
//! ```
//! or
//! ```
//! cargo run filename
//! ```
//! 
//! where: `filename` is a yaml file containing the DFA definition
//! 
//! # Output
//! 
//! To `stderr`: Debug display of the internal graph structure
//! 
//! To `stdout`: Graphviz definitions of the graph structure
use std::io::Write;

mod dfa;
mod graph;

// *********************************************************************
fn main() {

    // Get and validate the filename on the command line
    let filename = get_filename(std::env::args());

    // Load the yaml file getting a Box pointing to a DFA
    // instance on the heap
    let dfa = dfa::DFA::new_from_file(&filename);

    // Validate the DFA
    dfa.validate().expect("Validation Failure:");

    // Get a state structure for the DFA
    let graph = graph::Graph::new_from_dfa(&dfa);

    // Write the debug version of the graph to stderr
    eprintln!("{:?}", graph);

    // Write the Graphviz version of the graph to stdout
    println!{"{}", graph};

}

// *********************************************************************
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