use std::time::Duration;

use tokio::{join, task, time::{sleep, Instant}};

async fn download_with_retry(url: &str, retries: u8) -> Result<String, reqwest::Error> {
    for attempt in 1..=retries {
        match reqwest::get(url).await {
            Ok(resp) => return resp.text().await,
            Err(e) => {
                println!("Attempt {} failed: {}", attempt, e);
                if attempt < retries {
                    sleep(Duration::from_secs(2)).await;
                } else {
                    return Err(e);
                }
            }
        }
    }
    unreachable!()
}

pub async fn downloads() {
    let start = Instant::now();
    let (res1, res2) = join!(
        download_with_retry("https://httpbin.org/ip",3),
        download_with_retry("https://wrsddsf",3) //wrong url for generating error
    );

    for  (i, res) in [res1, res2].iter().enumerate() {
        match res {
            Ok(content) => {
                println!("Response {i} len: {:?}", content);
            }
            Err(err) => {
                println!("Response {i} len: {:?}", err);
            }
        }
    }
    println!("Elapsed: {:?}", start.elapsed());
}

pub async fn spwan_downloads() {
    let start = Instant::now();
    let urls = vec!["https://httpbin.org/ip", "https://wrsddsf"];
    let mut handles = vec![];
    for url in urls {
        let url = url.to_string();
        let handle = task::spawn(async move {
            download_with_retry(&url,3).await.map(|r| format!("{} -> len {}", url, r.len()))
        });
        handles.push(handle);
    }
    let mut results = vec![];

    for handle in handles {
        results.push(handle.await.unwrap());
    }

    for result in results {
        match result {
            Ok(msg) => println!("ok {}", msg),
            Err(e) => eprintln!("error {}", e),
        }
    }
    println!("Elapsed: {:?}", start.elapsed());

}
