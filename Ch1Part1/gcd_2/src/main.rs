// These pull in what are known as traits in Rust

// https://doc.rust-lang.org/std/macro.write.html
use std::io::Write;

// https://doc.rust-lang.org/std/str/trait.FromStr.html
use std::str::FromStr;

//Process the command line
fn main() {

    let mut numbers = Vec::new();

    // Process the arguments from the command line, the first argument is
    // always the command that is being executed, so skip it.
    for arg in std::env::args().skip(1) {

        // The expect function process a Result object returned from
        // the from_str call. If the result is OK, it's returned, if's it 
        // Err, the message is pinted to stderr and the program panics
        numbers.push(u64::from_str(&arg)
            .expect("error parsing argument"));

    }

    // Need to have at least one number included
    if numbers.len() == 0 {

        // Unwrap is also used to handle Result objects. It's simpler than
        // expect in that is returns the value if it's OK or just panic's without
        // a message otherwise
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...")
            .unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);


}

// Determine and return the greatest common denominator between two 64 bit integers

fn gcd (mut n: u64, mut m: u64) -> u64 {
    
    // Make sure the parameters are valid
    assert!(n != 0 && m != 0);

    while m != 0 {

        if m < n {
            let t = m;
            m = n;
            n = t;
        }

        m = m % n;

    }

    // A functions return value is, by default the result of the last expression
    n

}

// You can have multiple functions with the test decoration
#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19),  3 * 11);
}