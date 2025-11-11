pub enum Number {
    Float(f32),
    Int(u8),
}

//string operation & functions
pub fn string_op() {
    let s1 = "hello";
    let mut s2 = String::from("world"); // growable heap allocated string
    s2.push_str(" there");
    println!("{}", s2);
    let s3 = String::from("hello ") + &s2 + &s1;
    println!("{} {}", s3, s2);
    // slice string
    let slice = &s3[0..5];
    println!("{}", slice);
    // Iterate over string
    for c in s3[0..2].chars() {
        println!("{}", c);
    }
    // Other string functions
    println!(
        "{} {} {} {} {} {} {}",
        s1.len(),
        s1.is_empty(),
        s1.contains("llo"),
        s1.replace("h", "y"),
        s1.to_uppercase(),
        s1.trim(),
        s1.chars().rev().collect::<String>()
    );
    let char1 = s2.chars().nth(1);

    match char1 {
        Some(c) => println!("{}", c),
        None => println!("{}", " char not found"),
    }
}

pub fn add() -> u8 {
    let x: u8 = 10;
    let y: u8 = 20;
    return x + y;
}

pub fn add_float(x: &f32, y: &f32) -> f32 {
    return *x + *y;
}

pub fn add_num(x: &f32, y: &f32, is_float: bool) -> Number {
    if is_float {
        return Number::Float(add_float(&x, &y));
    } else {
        return Number::Int(add());
    }
}
// mutable by ref
pub fn mutable(x: &mut f32, y: &mut f32) -> f32 {
    *x += 1.9;
    *y += 0.1;
    return *x + *y;
}

pub fn mutable_heap() {
    let mut s1 = String::from("test");
    xyz(&mut s1);
    println!("{}", s1);
}

pub fn xyz(x: &mut String) {
    x.push_str("aaaa");
}
