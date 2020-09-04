// Compound types

fn main() {

    // Tuplies
    println!("\n**** Tuples");
    
    let t1 = (123, "Hi", b"frodo", 3.1415, '🐩');
    
    // Talk about the default formatter
    // println!("t1: {}", t1);
    println!("t1: {:?}, t1.2: {:?}", t1, t1.2);
    
    // Arrays
    println!("\n**** Arrays");
        
    let a1 = [0, 1, 2, 3, 4];
    println!("a1: {:?}{}", a1, type_of(&a1));
    
    // Not mutable, will fail
    // a1[2] = 3;
    
    let mut a2 = ['a', 'b', 'c', 'd', 'e'];
    println!("a2: {:?}{}", a2, type_of(&a2));
    
    a2[3] = 'q';
    println!("a2: {:?}{}", a2, type_of(&a2));
    
    let a3: [f64; 10];
    
    // Will fail, uninitialized variable
    // a3[0] = 12.1;
    
    let f1 = 3.1415;
    a3 = [f1 * 2.81; 10];
    println!("a3: {:?}{}", a3, type_of(&a3));
    
    // Will fail, out of bounds
    // println!("a3[10]: {:?}", a3[10]);

    // Vectors
    println!("\n**** Vectors");

    let mut v1 = Vec::new();
    // Will panic as length is initially zero
    // v1[0] = 1;
    v1.push(1);
    v1.push(2);
    
    // Will fail, mismatched types
    // v1.push("xx");
    
    print!("v1 - {:?} type:{} ", v1, type_of(&v1));
    println!("len: {}", v1.len());
    
    v1.resize(5, 23);
    print!("v1 - {:?} type:{} ", v1, type_of(&v1));
    println!("len: {}", v1.len());

    // Slices
    println!("\n**** Slices");
    
    let mut v2 = Vec::new();
    
    for i in 0..9 {
        v2.push(i);
    }
    
    print!("v2 - {:?} type:{} ", v2, type_of(&v2));
    println!("len: {}", v2.len());
    
    let sv2 = &v2[3..5];
    println!("sv2 - {:?} type:{} len:{} ", sv2, type_of(&sv2), sv2.len());
    
    v2[3] = 33;
    
    let sv2 = &v2[3..5];
    println!("sv2 - {:?} type:{} len:{} ", sv2, type_of(&sv2), sv2.len());
    
    // Struct
    println!("\n**** Structs");

    #[derive(Debug)]
    struct TupleStruct(i64, f64, String);
    
    let ts1 = TupleStruct(32, 3.1415, "Blah".to_string());
    println!("ts1 - {:?} type:{} {}", ts1, type_of(&ts1), ts1.2);
    
    #[derive(Debug)]
    struct CStruct {
        
        i1: u32,
        f1: f32,
        s1: String,
    }
    
    let cs1 = CStruct {i1: 22, f1: 2.81, s1: "BlahBlah".to_string()};
    println!("cs1 - {:?} type:{} {}", cs1, type_of(&cs1), cs1.f1);
            
}

// Print the type of a variable

fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}