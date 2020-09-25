//! CSIS-616 - Traits and Generics
//! 
//! Ralph W. Crosby PhD
//! 
//! 
//! # Usage
//! 
//!    ```shell
//!    cargo run
//!    ```
//! 
//! 

use std::fmt;
use std::ops;

/// A user defined trait
trait MyTrait {

    /// There is no implementation supplied so anyone
    /// implementing the trait will need to provide an
    /// implemention of this function
    fn get_string(&self) -> String;

    /// A default implementation is supplied for this
    /// function so a specific implementation isn't required
    fn another_fn(&self) -> u32 {
        10
    }

}

/// Rust will supply an implementation of the Debug trait
#[derive(Debug)]
struct SumMe {

    val1: u32,
    val2: isize,
    str1: &'static str

}


/// To produce a custom display we need to implement the Display trait and it's
/// `fmt` function for our structure
impl fmt::Display for SumMe {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        
        write!(f, "From My Display: {}-{}-{}", self.val1, self.val2, self.str1)

     }

}

/// Implement our custom trait
impl MyTrait for SumMe {

    fn get_string(&self) -> String {
        format!("From MyTrait: {}/{}/{}", self.val1, self.val2, self.str1)
    }

}

/// A simple generic function to add two values of the same type
/// that supports the `Add` trait. A value of the same type that
/// supports the `Add` trait will be returned.
fn summ_it<T> (v1: T, v2: T) -> T 
    where T: ops::Add<Output = T> 
{
    v1 + v2
}

fn main() {
    println!("Traits!");

    // Create a couple of instances of our structure, note they don't need 
    // to be mutable
    let sme1 = SumMe{val1: 23, val2: -54, str1: "Hello"};
    let sme2 = SumMe{val1: 100, val2: 54, str1: "World"};

    // Print the instances using the Debug trait format
    println!("{:?}", sme1);
    println!("{:?}", sme2);

    // Print the instances using the custom display format
    println!("{}", sme1);
    println!("{}", sme2);

    // Call our custom trait's function to get a string representation of the
    // structure
    let str2 = sme1.get_string();
    println!("{}", str2);

    // Call our generic function with values of a couple of types
    let i = summ_it(3, 5);
    let z = summ_it(3.14159, 5.5);
    println!("{} {}", i, z);

}
