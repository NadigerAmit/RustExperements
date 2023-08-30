use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};
// Approach 1 : using the asyns.
#[tokio::main]
async fn main() {
    println!("requesting from the app");
    let result = app().await;
    println!("Result: {}", result);
}

async fn app() -> i32 {
    // Your asynchronous code here
    sleep(Duration::from_secs(3)).await;
    101
}

// comment above code an Uncomment below code if you want to use approach 2 
// Approach 2 : using the Runtime::

/*
fn main() {
    let mut rt = Runtime::new().unwrap();
    let future = app();
    println!("Requesting the app ");
    let result = rt.block_on(future);
    println!("Result: {}", result);
}

async fn app() -> i32 {
    sleep(Duration::from_secs(3)).await;
    101
}
*/

/*
Op => 
requesting from the app
Result: 101
*/