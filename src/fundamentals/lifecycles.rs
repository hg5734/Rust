struct Sentence<'a> {
    text: &'a str,
}

pub fn lifecycle_test() {
    let s = String::from("Rust lifetimes are fun");
    let sent = Sentence { text: &s };
    println!("{}", sent.text);
}