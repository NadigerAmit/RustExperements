use futures::future::FutureExt; // Import the extension traits
use futures::future::TryFutureExt; // Import the extension traits
use tokio::time::{sleep, Duration};

async fn async_operation() -> i32 {
    sleep(Duration::from_secs(3)).await;
    101
}

#[tokio::main]
async fn main() {
    let future = async_operation().map(|result| {
        if result > 20 {
            Ok("Value is greater than 20")
        } else {
            Err("Value is not greater than 20")
        }
    });
    println!("Continuing in main thread execution");
    // Await the result of the future1 and print it.
    match future.await {
        Ok(str) => {
                println!("OK => {}", str);
        }
        Err(str) => {
                println!("Err => {}", str);
        }
    }
}