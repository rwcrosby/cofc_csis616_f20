// Experiment with some basic scalar values

fn main() {

    // Integers
    println!("\n**** Integers");

    let i1 = 23;
    let mut i2 : u64 = 2;

    for _ in 0..10 {
        i2 *= 2;
    }

    println!("i1: {}:{}, i2 {}:{}", i1, type_of(&i1), i2, type_of(&i2));

    // Floats
    println!("\n**** Floats");
    
    let f1 = 123.45;
    let mut f2 = 2e-2;
    
    for i3 in 0..10 {
        // try f2 += i3 first
        // Show the result when you use f64 instead of f32
        f2 += i3 as f64;
    }
    
    let f3 = 123 as f32;
    
    println!("f1: {}:{}, f2 {}:{}, f3 {}:{}", f1, type_of(&f1), f2, type_of(&f2), f3, type_of(&f3));
    
    // Booleans
    println!("\n**** Booleans");

    let b1 = true;
    let mut b2 = false;
    
    for _ in 0..9 {
        b2 = !b2;
    }
    
    println!("b1: {}:{}, b2 {}:{}", b1, type_of(&b1), b2, type_of(&b2));

    // Passing scalars to functions
    println!("\n**** Passing scalars to functions");
    
    let i3 = 12;
    println!("fn1: {}, i3 {}", fn1(i3), i3);

    let i4 = 42;
    println!("i4: {}, fn2: {}, i4 {}", i4, fn2(i4), i4);
    
    let ri5 = &mut 100;
    let i6 = fn3(ri5);
    println!("ri5: {}:{}, i6 {}:{}", ri5, type_of(&ri5), i6, type_of(&i6));
   
    *ri5 += 10;
    println!("ri5 incremented: {}:{}", ri5, type_of(&ri5));
    
    // Mutablity
    println!("\n**** Mutability");

    let mut i7 = 128;
    
    let ri7 = &mut i7;

    *ri7 *= 2;

    print!("ri7: {}, ", *ri7);

    i7 *= 2;

    println!("i7: {}", i7);


}

// Parameters as values

fn fn1 (p1: u32) -> u32 {

    p1 / 2

}

// Mutable

fn fn2 (mut p1: u32) -> u32 {

    p1 = p1 / 2;
    p1

}

// Parameters as references

fn fn3 (p1: &mut u32) -> u32 {

    *p1 = *p1 / 2;
    *p1 / 2

}

// Print the type of a variable

fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}