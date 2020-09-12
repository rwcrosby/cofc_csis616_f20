//! # CSIS-616 Homework #1 - Reference Solution
//! 
//! Dr. Ralph Crosby (crosbyrw@cofc.edu)

use std::io::Write;

/// Create a graphviz definition for a simple graph
/// 
/// # Usage
/// 
///    ```
///     homework_1 states
///     ```
/// 
///    where: `states` is a comma-separated list of state names
/// 
/// # Output
/// 
/// A `graphviz` definition is produced to `stdout`. To generate
/// a graphics file use something like
/// 
/// ```
/// cargo run a,b,d > test.dot
/// dot -Tpdf -otest.pdf < test.dot
/// ```
fn main() {

    // Get the arguments as a vector
    let args: Vec<String> = std::env::args().collect();
    
    // Make sure only one argument was passed
    if args.len() != 2 {
        writeln!(std::io::stderr(), "Usage: hw1 statelist")
            .unwrap();
        std::process::exit(1);
    }

    gen_graph(&args[1]);

}

fn gen_graph(graph: &str) {

    // Split the state list on the comma generating a vector of
    // str references
    let states: Vec<&str> = graph.split(',').collect();

    // Print the graphviz header
    println!("digraph {{
        rankdir=LR;
        node [shape=point]; start;
        node [shape=doublecircle]; {}
        node [shape=circle];", states.last().unwrap());

    // Start with our dummy node
    let mut prior = "start";

    // Loop through the states
    for state in states {
        println!("\t{} -> {};", prior, state);
        prior = state;
    }

    // Print the closing for the digraph
    println!("}}");

}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test1() {
        gen_graph("a,b,c");
    }

}