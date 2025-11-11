use std::{sync::{mpsc, Arc, Mutex}, thread, time::Duration};

enum Message {
    Integer(i32),
    Text(String),
}

pub fn multithread_test() {
      basic_child_thread();
      moving_ownership();
      message_passing();
      shared_state();
      thread_pool();

}

fn thread_pool() {
    let (tx, rx ) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    for id in 1..3 {
        let rx = Arc::clone(&rx);
        thread::spawn(move||loop {
            let job = rx.lock().unwrap().recv();
            match job {
                Ok(task) =>{
                    println!("worker{id} task {task}");
                    thread::sleep(Duration::from_secs(1));
                },
                Err(e) => {
                    println!("err {}", e);
                    break;
                }
            }
        });
        for i in 1..6 {
            tx.send(format!("Job {}", i)).unwrap();
        }
        thread::sleep(std::time::Duration::from_secs(5));
    }

}

fn shared_state() {
    let msg = Arc::new(Mutex::new(String::from("aa")));
    let mut handles = vec![];
    for i in 0..5 {
        let counter_clone = Arc::clone(&msg);
        let handle = thread::spawn(move || {
            let mut msg1 = counter_clone.lock().unwrap();
            msg1.push_str(&i.to_string())
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final count: {}", *msg.lock().unwrap());
}

fn message_passing() {
    let (tx , rx ) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let msgs = vec![1, 2, 3];
        for msg in msgs {
            tx1.send(Message::Integer(msg)).unwrap();
        }
    });
    thread::spawn(move ||{
        
        let msgs = vec!["hi", "hello", "how"];
        for msg in msgs {
            tx.send(Message::Text(msg.to_string())).unwrap();
        }
    });
    for rec in rx {
        match rec {
            Message::Integer(num) => println!("Received integer: {}", num),
            Message::Text(text) => println!("Received text: {}", text),
        }
    }
 
}

fn moving_ownership() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

fn basic_child_thread() {
    let child_thread = thread::spawn(|| {
        for i in 1..5 {
            println!("child thread itr {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });
    let child_thread2 = thread::spawn(|| {
        for i in 1..5 {
            println!("child thread 2 itr {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });
   
    for i in 1..4 {
        println!("parent thread itr {}", i);
        thread::sleep(Duration::from_secs(1))
    }
    child_thread.join().unwrap();
    child_thread2.join().unwrap();

}
