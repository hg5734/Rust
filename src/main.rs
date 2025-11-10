mod basic;
mod traits;
mod structs_enum;
mod collections;

fn main() {
    test_op();
}

fn test_op() {
    println!("Hello, world!");
    println!("add fn {}", basic::add());
    let mut x: f32 = 10.1;
    let mut y: f32 = 20.9;
    println!("add float {}", basic::add_float(&10.1, &33.3));
    match basic::add_num(&x, &y, false) {
        basic::Number::Float(v) => println!("float result {} ", v),
        basic::Number::Int(v) => println!("int result {} ", v),
    }
    println!("add float {}", basic::mutable(&mut x, &mut y));
    println!("{} {}", x, y);
    // String operations
    basic::string_op();
    basic::mutable_heap();
    // Struct Operations
    structs_enum::stuct_operations();
    //Enum Operations
    structs_enum::enum_operations();
    //Collections
    collections::collections();
    //trait
    traits::trait_test();
}
