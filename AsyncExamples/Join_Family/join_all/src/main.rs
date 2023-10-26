use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use futures::future::join_all;

async fn increment_counter(counter: Arc<Mutex<i32>>,task_id:u8,timeout:u32) ->i32 {
    let mut ctr:i32 = 0;
    {
        let mut guard = counter.lock().unwrap();
        *guard += 1;
        ctr = *guard;
    }
    sleep(Duration::from_secs(timeout.into())).await;
    // Perform other asynchronous operations...
    println!("Counter incremented with TaskId {} and counter _val = {}",task_id,ctr);
    ctr

}

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));

    let tasks = vec![
        increment_counter(Arc::clone(&counter),0,5),
        increment_counter(Arc::clone(&counter),1,2),
    ];

    let result = join_all(tasks).await;
    println!("result = {:?}",result);

    let final_value = counter.lock().unwrap();
    println!("Final counter value: {}", *final_value);
} 
/*

     Running `target/debug/join_all`
Counter incremented with TaskId 1 and counter _val = 2
Counter incremented with TaskId 0 and counter _val = 1
result = [1, 2]
Final counter value: 2

*/