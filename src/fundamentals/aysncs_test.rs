use std::time::Duration;

use tokio::{
    join,
    task::{self, JoinHandle},
    time::sleep,
};

async fn hello() -> String {
    "hello future generated".to_string()
}

async fn task_fn(x: u8) -> JoinHandle<()> {
    return task::spawn(async move {
        sleep(Duration::from_secs(1)).await;
        println!("Task {} done", x);
    });
}

//Error handling ex
async fn might_fail(success: bool) -> Result<String, String> {
    sleep(Duration::from_secs(1)).await;
    if success {
        Ok("Success!".to_string())
    } else {
        Err("Something went wrong".to_string())
    }
}

pub async fn test() {
    println!("{}", hello().await);
    //call multiple future together
    join!(task_fn(3), task_fn(4));
    // parallel execution
    task_fn(1).await;
    task_fn(2).await;
    sleep(Duration::from_secs(2)).await;
    println!("Both done!");

    match might_fail(false).await {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("{}", e),
    }
}
