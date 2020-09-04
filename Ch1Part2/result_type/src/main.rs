// The result type

fn main() {

    // The result type
    println!("\n**** The Result type");

    // Need to tell rust what the type of the value is
    // let r1 = Ok("This is cool");
    
    let r1: Result<&str, &str> = Ok("This is cool");
    
    println!("r1: {} = {:?}", type_of(&r1), r1);
    
    let r2: Result<i64, &str> = Err("Woops");
    println!("r2: {} = {:?}", type_of(&r2), r2);
    
    assert_eq!(r2.is_ok(), false );
        
    // Pattern matching
    println!("\n**** Pattern matching");

    match r2 {
        Ok(x) => println!("r2 OK = {} ", x),
        Err(m) =>  println!("r2 Err = {}", m),
    }
        
    // Helpers
    println!("\n**** Helpers");

    let v1 = r1.expect("Are we ok?");
    println!("v1: {} = {}", type_of(&v1), v1);
    
    let v1 = r1.unwrap();
    println!("v1: {} = {}", type_of(&v1), v1);
    
    let v2 = r2.expect_err("Blah");
    println!("v2: {} = {}", type_of(&v2), v2);  

}

// Print the type of a variable

fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}