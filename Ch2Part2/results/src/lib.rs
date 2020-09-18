//! CSIS-616 - Results
//! 
//! Ralph W. Crosby PhD.
//! 
//! 
//! # Usage
//! 
//!    ```shell
//!    cargo test
//!    ```
//! 

#[allow(dead_code)]
/// Return a result based on the input parameter
/// either Ok or Err 
/// 
/// The return types for Ok and Err demonstrate two ways of returning
/// unicode strings. If all that's needed is a static string that will
/// not be modified, a referenece to a static slice is more efficient.
fn return_result(tf : bool) -> Result<&'static str, String> {
    
    if tf {
        Ok("Is OK man")
    } else {
        Err("Not OK".to_string())
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn result_1() {

        // Get an Ok return code from the test function
        assert!(match return_result(true) {
                    Ok(msg) => {
                        println!("Ok: {}", msg);
                        true
                    }
                    Err(_) => false
                });

    }

    #[test]
    fn result_2() {

        // Get an Err return code from the test function and
        // assign the variable r the value true if Err is returned
        let r = if let Err(msg) = return_result(false) {
            println!("Error: {}", msg);
            true
        } else {
            false
        };

        assert!(r);

    }

}
