use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

use futures::future::join;

async fn task1() -> i32 {
    // Perform some asynchronous computation
    sleep(Duration::from_secs(3)).await;
    101
}

async fn task2() -> f64 {
    // Perform another asynchronous computation
    sleep(Duration::from_secs(5)).await;
    3.14
}

#[tokio::main]
async fn main() {
    let (result1, result2) = join(task1(), task2()).await;
    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
}

/*
Running `/home/amit/OmPracticeRust/AsyncExperements/Ardan-1HourAsync/target/debug/hello_tokio`
Result 1: 101
Result 2: 3.14

// Please note that the result will be obtained after 5 sec at once , even though result is obtained after 3 secs. 
*/