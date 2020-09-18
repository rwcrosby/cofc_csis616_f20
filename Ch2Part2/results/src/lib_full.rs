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

        let r = if let Err(msg) = return_result(false) {
            println!("Error: {}", msg);
            true
        } else {
            false
        };

        assert!(r);

    }

}
