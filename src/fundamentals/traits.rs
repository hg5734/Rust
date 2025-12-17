use std::collections::HashMap;

struct MyBank {
    accounts: HashMap<String, u16>,
}
trait Bank {
    fn deposit(&mut self, name: &str, amount: u16);
    fn balance(&self, name: &str) -> u16;
}

impl Bank for MyBank {
    fn deposit(&mut self, name: &str, amount: u16) {
        let entry = self.accounts.entry(name.to_string()).or_insert(0);
        *entry += amount;
    }

    fn balance(&self, name: &str) -> u16 {
        *self.accounts.get(name).unwrap_or(&0)
    }
}

pub fn trait_test() {
    let mut bank = MyBank {
        accounts: HashMap::new(),
    };
    bank.deposit("Himanshu", 100);
    bank.deposit("Himanshu", 50);

    println!("Balance: {}", bank.balance("Himanshu")); // 150
    println!("Balance (unknown): {}", bank.balance("Alice")); // 0
}
