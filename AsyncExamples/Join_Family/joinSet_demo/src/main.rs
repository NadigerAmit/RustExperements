use std::sync::{Arc, Mutex};
use tokio::task::JoinSet;

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

    // Using Tokio JoinSet
    let mut join_set = JoinSet::new();
    for i in 0..10 {
        join_set.spawn(increment_counter(Arc::clone(&counter)));
    }

    while let Some(res) = join_set.join_next().await {
        println!("{res:?}");
    }

    let final_value = counter.lock().unwrap();
    println!("Final counter value: {}", *final_value);
} 
/*
warning: `joinSet_demo` (bin "joinSet_demo") generated 2 warnings (run `cargo fix --bin "joinSet_demo"` to apply 1 suggestion)
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/joinSet_demo`
Counter incremented
Counter incremented
Ok(())
Ok(())
Counter incremented
Counter incremented
Ok(())
Ok(())
Counter incremented
Counter incremented
Ok(())
Ok(())
Counter incremented
Ok(())
Counter incremented
Ok(())
Counter incremented
Counter incremented
Ok(())
Ok(())
Final counter value: 10
*/