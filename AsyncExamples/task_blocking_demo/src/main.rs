use tokio::time::Duration;
use tokio::task;

async fn async_task() {
    for i in 0..5 {
        println!("Async Task: Jai Shree Ram - Count {}", i);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

fn blocking_task() {
    for j in 0..5 {
        println!("Blocking Task:Jai Bajrang Bali - Count {}", j);
        std::thread::sleep(Duration::from_secs(1))
    }
}

#[tokio::main]
async fn main() {
    // Spawn an asynchronous task (detached)
    let async_handle = tokio::spawn(async_task());

    // Spawn a non-detached task using spawn_blocking
    let blocking_handle = tokio::task::spawn_blocking(|| {
        blocking_task();
    });

    // Main task continues executing concurrently with async_handle and blocking_handle
    for k in 0..5 {
        println!("Main Task: Count {}", k);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    // Wait for the async_task to complete (optional)
    async_handle.await.expect("Async Task panicked");

    // The blocking_task automatically runs to completion

    println!("All tasks have completed.");
}
