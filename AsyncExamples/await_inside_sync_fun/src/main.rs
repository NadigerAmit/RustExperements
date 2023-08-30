use tokio::time::{self, Duration};

async fn async_operation() -> i32 {
    println!("Start async operation");
    time::sleep(Duration::from_secs(2)).await; // Use time::sleep instead of sleep
    println!("Async operation completed");
    101
  }
  
  fn synchronous_function() {
    println!("Start synchronous function");
    let runtime = tokio::runtime::Builder::new_current_thread()
      .enable_time() // Enable timers
      .build()
      .unwrap();
    let result = runtime.block_on(async {
      let value = async_operation().await;
      value
    });
    println!("Synchronous function completed with value: {:?}", result);
  }
  
  fn main() {
    synchronous_function();
  }

  /*
  Op => 
    Running `target/debug/await_inside_sync_fun`
    Start synchronous function
    Start async operation
    Async operation completed
    Synchronous function completed with value: 101
  */