use std::fs;
struct Person {
    username: String,
    email: String,
    age: u8,
}

impl Person {
    fn new(username: String, email: String) -> Self {
        Self {
            username,
            email,
            age: 10,
        }
    }

    fn age(&self) {
        println!("age of person is {}", self.age);
    }

    fn ownership(self) {
        println!("ownership moved");
    }

    fn add_age(&mut self, age: u8) {
        self.age = self.age + age;
    }
}

pub fn stuct_operations() {
    let mut person = Person {
        username: String::from("hg5734"),
        email: String::from("hg5734"),
        age: 30,
    };
    person.email = String::from("hg1@gmail.com");
    println!("{} {}", person.username, person.email);
    person.age();
    person.ownership();
    let mut person2 = Person::new(String::from("hg5734"), String::from("hg5734"));
    person2.age();
    person2.add_age(5);
    person2.age();
}

//Enum practice
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn print(&self) {
        match self {
            Message::Quit => println!("quite"),
            Message::Move { x, y } => println!("move {} {}", x, y),
            Message::ChangeColor(x, y, z) => println!("move {} {} {}", x, y, z),
            Message::Write(x) => println!("move {}", x),
        }
    }
}

pub fn is_even(x: u32) -> Option<bool> {
    if x % 2 == 0 {
        return Some(true);
    }
    return None;
}

pub fn enum_operations() {
    let msgs = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
        Message::ChangeColor(255, 0, 0),
    ];
    for msg in msgs {
        msg.print();
    }

    // error handling
    let result = fs::read_to_string("wrongpath.pdf");
    match result {
        Ok(file) => println!("file {}", file),
        Err(e) => println!("file {}", e),
    }

    match is_even(21) {
        Some(result) => println!("is even {}", result),
        None => println!("not even"),
    }
}
