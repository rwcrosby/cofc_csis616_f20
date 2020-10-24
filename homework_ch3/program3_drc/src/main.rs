//! CSIS-616 - Program #3
//! 
//! Ralph W. Crosby PhD.
//! 
//! 
//! # Usage
//! 
//! ```
//! program3_drc filename
//! ```
//! or
//! ```
//! cargo run filename
//! ```
//! 
//! where: `filename` is a yaml file containing the DFA definition
//! 
//! Program will read in strings from stdin and process each line as a potential
//! sentence in the language.
//! 
//! # Output
//! 
//! To `stderr`: Debug display of the internal graph structure
//! 
//! To `stdout`: Graphviz definitions of the graph structure

use serde::{Deserialize};
use std::io;
use std::io::prelude::*;
use std::io::Write;

// *********************************************************************
/// # Deterministic Finite Automata Structure
/// 
/// Create a structure that the YAML files will be deserialized into.
/// Note the use of the `Deserialize` trait
/// 
#[derive(Debug, Deserialize)]
struct DFA {

    /// The set of characters comprising the alphabet
    alphabet: Vec<char>,

    /// State number (1 relative) for the start state
    start: usize,

    /// Set of accept states (1 relative)
    accept: Vec<usize>,

    /// Matrix of transitions, rows are states, columns characters in the alphabet
    transitions: Vec<Vec<usize>>,
    
}

// *********************************************************************
/// # Definition of a single state
#[derive(Debug)]
struct State {

    /// Is this an accept state
    accept_state: bool,

    /// Set of transitions (0 relative)
    transitions: Vec<usize>

}

// *********************************************************************
/// # State based representation of the DFA
#[derive(Debug)]
struct StateGraph {

    /// The set of characters comprising the alphabet
    alphabet: Vec<char>,

    /// State number (0 relative) for the start state
    start_state: usize,

    /// Vector of state objects
    states: Vec<Box<State>>

}

// *********************************************************************
fn main() {

    // Get and validat the filename on the command line
    let filename = get_filename(std::env::args());

    // Load the yaml file getting a Box pointing to a DFA
    // instance on the heap
    let dfa = DFA::new_from_file(&filename);

    // Validate the DFA
    dfa.validate().expect("Validation Failure:");

    // Get a state structure for the DFA
    let state_graph = StateGraph::new_from_dfa(&dfa);

    eprintln!("{:?}", state_graph);

    state_graph.write_graphviz();

    // Process through the input until end of file (cntl-z) is encountered
    let stdin = io::stdin();
    for line in stdin.lock().lines() {

        // Get the line out of the Result, should never error
        let sentence = &line.unwrap();
        println!("Processing sentence <{}>", sentence);

        match state_graph.test_sentence(sentence) {
            Ok(b) => println!("Sentence {} in the language", 
                              if b {"is"} else {"is not"}),
            Err(s) => println!("Error processing sentence: {}", s)
        }

    }

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

// *********************************************************************
/// Implement the methods of the DFA structure
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

    /// Validate the correctness of the DFA
    fn validate(&self) -> Result<(), String> {

        // The number of characters in the alphabet should match the number
        // of columns in each state row

        for (rnum, row) in self.transitions.iter().enumerate() {

            if row.len() != self.alphabet.len() {
                return Err(format!("Wrong number of columns({}) in row {}, should be {}",
                                    row.len(), rnum + 1, self.alphabet.len() ))
            }

        }

        // Validate that all states in the transition table are valid
        for (rnum, row) in self.transitions.iter().enumerate() {
            for (cnum, state) in row.iter().enumerate() {

                if *state as usize >  self.transitions.len() {
                    return Err(format!("Invalid transition state({}) in row {}, column {}",
                                        state, rnum + 1, cnum + 1 ))
                }
    
            }
        }

        // The start and accept states must be valid
        if self.start as usize > self.transitions.len() {
            return Err(format!("Start state({}), is not valid", self.start))
        }

        for acc_state in self.accept.iter() {
            if *acc_state as usize  > self.transitions.len() {
                return Err(format!("Accept state({}), is not valid", acc_state))
            }
        }

        Ok(())
    }

}

// *********************************************************************
/// Implement the methods of the State Graph structure
impl StateGraph<> {

    /// Create a state graph from a DFA structure
    fn new_from_dfa(dfa: &DFA) -> Box<StateGraph> {

        // Create an empty graph object
        let mut graph = Box::new(StateGraph{alphabet: dfa.alphabet.clone(), 
                                            start_state: dfa.start - 1,
                                            states: vec!() });

        // Look through the transition table building state objects
        for row in dfa.transitions.iter() {
            let mut v = Box::new(State{accept_state: false, transitions: vec!()});
            for col in row {
                v.transitions.push(col-1);
            } 
            graph.states.push(v);
        }    

        // Set the accept states
        for astate in dfa.accept.iter() {
            graph.states[*astate - 1].accept_state = true;
        }

        graph

    }

    /// Execute the graph on a sentence
    /// Return Err if a character not in the alphabet is encountered
    /// Return Ok and a bool indicating accept (true) or reject (false)
    fn test_sentence(&self, sentence: &str) -> Result<bool, String> {

        let mut state = self.start_state;

        for ch in sentence.chars() {

            let state_no = match self.alphabet.iter().position(|v| *v == ch) {
                Some(t) => t,
                None => return Err(format!("Character <{}> not in alphabet", ch))
            };

            print!("δ(q{}, {}) → ", state+1, ch);
            state = self.states[state].transitions[state_no];
            println!("(q{})", state+1);

        }

        Ok(self.states[state].accept_state)
    }

    /// Write the graph to stdout
    fn write_graphviz(&self) {

        println!("digraph {{");
        println!("\trankdir=LR;");
        println!("\tnode [shape=point]; start;");
        
        for (n, state) in self.states.iter().enumerate() {
            if state.accept_state {
                println!("\tnode [shape=doublecircle]; q{};", n+1);
            }
        }
        
        println!("\tnode [shape=circle];");
        println!("\tstart -> q{}", self.start_state+1);

        for (n, state) in self.states.iter().enumerate() {

            for (i, ch) in self.alphabet.iter().enumerate() {
                println!("\tq{} -> q{} [label=\"{}\"];", n+1, state.transitions[i] + 1, ch);
            }

        }

        println!("}}");

    }

}