use tokio::time::{sleep, Duration};
use futures::future::join;

async fn async_operation_1() -> i32 {
    println!("Start async operation 1");

    sleep(Duration::from_secs(2)).await;
    println!("Async operation 1 completed");
    101
}

async fn async_operation_2() -> String {
    println!("Start async operation 2");
    sleep(Duration::from_secs(3)).await;
    println!("Async operation 2 completed");
    "Jai Shree Ram, Jai Bajrang bali - Async!".to_string()
}

#[tokio::main]
async fn main() {
    println!("Start of main function");

    let future1 = async_operation_1();
    let future2 = async_operation_2();

    // Use the `join` combinator to run both futures concurrently.
    let result = join(future1, future2).await;

    // Extract results from the futures.
    let result1 = result.0;
    let result2 = result.1;

    println!("Result of async operation 1: {}", result1);
    println!("Result of async operation 2: {}", result2);

    println!("End of main function");
}