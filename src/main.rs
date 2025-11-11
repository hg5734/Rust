mod fundamentals;

fn main() {
    test_op();
}

fn test_op() {
    println!("Hello, world!");
    println!("add fn {}", fundamentals::basic::add());
    let mut x: f32 = 10.1;
    let mut y: f32 = 20.9;
    println!("add float {}", fundamentals::basic::add_float(&10.1, &33.3));
    match fundamentals::basic::add_num(&x, &y, false) {
        fundamentals::basic::Number::Float(v) => println!("float result {} ", v),
        fundamentals::basic::Number::Int(v) => println!("int result {} ", v),
    }
    println!("add float {}", fundamentals::basic::mutable(&mut x, &mut y));
    println!("{} {}", x, y);
    // String operations
    fundamentals::basic::string_op();
    fundamentals::basic::mutable_heap();
    // Struct Operations
    fundamentals::structs_enum::stuct_operations();
    //Enum Operations
    fundamentals::structs_enum::enum_operations();
    //Collections
    fundamentals::collections::collections();
    //trait
    fundamentals::traits::trait_test();
    //lifecycles
    fundamentals::lifecycles::lifecycle_test();
    // multithread
    fundamentals::multithread::multithread_test();
}
