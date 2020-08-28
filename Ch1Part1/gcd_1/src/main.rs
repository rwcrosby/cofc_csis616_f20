// Note the main program here isn't really used.
fn main() {
    println!("Hello, world!");
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