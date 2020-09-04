// Strings

fn main() {

    // Characters
    println!("\n**** Characters");

    let ch1 = "a";
    let ch2 = "🍕";
    println!("ch1 - {}:{:X?}:{}", ch1, ch1.as_bytes(), std::mem::size_of_val(ch1));
    println!("ch2 - {}:{:X?}:{}", ch2, ch2.as_bytes(), std::mem::size_of_val(ch2));
    
    // Byte strings
    println!("\n**** Byte strings");

    let bs1 = b"hello";
    println!("bs1 - {:?}:{:X?}:{}", bs1, bs1, std::mem::size_of_val(bs1));
    
    // Strings and string slice references
    println!("\n**** Strings and str");

    let sr1 = "Hello Class 🏛";
    println!("sr1: {} = /{}/", type_of(&sr1), sr1);
    
    let sr2 = &sr1[5..7];
    println!("sr2: {} = /{}/", type_of(&sr2), sr2);
    
    for sr3 in "This is a sentence in some language".split(" ") {
        println!("sr3: {} = /{:?}/", type_of(&sr3), sr3);
    }

    let s1 = "A real string".to_string();
    println!("s1: {} = /{}/", type_of(&s1), s1);
    
    

}

// Print the type of a variable

fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}