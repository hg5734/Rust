use std::{collections::HashMap, fmt::Debug};

pub fn collections() {
    //vectors
    let mut vec = vec![]; //Vec::new (); 
    vec.push("hello");
    vec.push("world");
    vec.remove(1);
    println!("{:?}", vec);
    //hashmaps
    let mut users = HashMap::new();
    users.insert(1, "himanshu");
    println!("{:?}", users);
    println!("{}", users.get(&1).map_or("not found", |v| v));

    let vector1 = vec![1, 2, 3];
    println!("{}", vector1.iter().sum::<i32>());
    println!("{:?}", vector1.iter().filter(|x| *x % 2 == 0).next());
    println!("{:?}", vector1.iter().map(|x| *x + 1).next());
    //generic
    print_debug(42);
    print_debug("Hello");
}

//generic
fn print_debug<T: Debug>(x: T) {
    println!("{:?}", x);
}
