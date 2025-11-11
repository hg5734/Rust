use std::{thread, time::Duration};



pub fn multithread_test() {
    let child_thread = thread::spawn(||{
        for i in 1..5 {
            println!("child thread itr {}", i);
            thread::sleep(Duration::from_secs(3));
        }
    });
    for i in 1..4 {
        println!("parent thread itr {}", i);
        thread::sleep(Duration::from_secs(2))
    }
    child_thread.join().unwrap();

}