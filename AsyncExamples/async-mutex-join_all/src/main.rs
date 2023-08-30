use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use futures::future::join_all;

async fn increment_counter(counter: Arc<Mutex<i32>>) {
  {
    let mut guard = counter.lock().unwrap();
    *guard += 1;
  }

  // Perform other asynchronous operations...
  println!("Counter incremented");
}

#[tokio::main]
async fn main() {
  let counter = Arc::new(Mutex::new(0));

  let tasks = vec![
    increment_counter(Arc::clone(&counter)),
    increment_counter(Arc::clone(&counter)),
  ];

  join_all(tasks).await;

  let final_value = counter.lock().unwrap();
  println!("Final counter value: {}", *final_value);
} 