//! CSIS-616 - Enums example
//! 
//! Ralph W. Crosby PhD.
//! 
//! # Usage
//! 
//!    ```shell,
//!    cargo test
//!    ```
//! 

// The two traits are required by the assert macros used in the tests.
// The Debug trait allows for debug mode printing of Colors
// The PartialEq train allows basic equality comparisons of Colors
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum Colors {
    Red,
    Green,
    Orange
}

#[allow(dead_code)]
/// This demonstrates the ability to attach different data to the
/// legs of an enum
enum Foods {
    Fruit(Colors),
    Animal(&'static str)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn enum_1() {

        let t = Colors::Red;
        assert_eq!(t, Colors::Red);
    }

    #[test]
    fn enum_2() {

        let orange = Foods::Fruit(Colors::Orange);
        let shrimp = Foods::Animal("Pandalus borealis"); 

        // We're using the match much line a case statement in other
        // languages. All possible values of the enum must be accounted
        // for or Rust will generate a compiler error.
        match orange {
            Foods::Fruit(c) => assert_eq!(c, Colors::Orange),
            Foods::Animal(_) => panic!("It's not an animal")
        }

        match shrimp {
            Foods::Fruit(_) => panic!("It's not a fruit"),
            Foods::Animal(s) => assert_eq!(s, "Pandalus borealis")
        }

    }

}
