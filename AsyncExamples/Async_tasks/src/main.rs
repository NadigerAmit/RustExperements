use tokio::time::{Duration, sleep};

async fn task_with_delay(id: u32) {
    println!("Task {} started", id);
    // Simulate some async work (e.g., fetching data from a web API)
    sleep(Duration::from_secs(2)).await;
    println!("Task {} completed", id);
}

#[tokio::main]
async fn main() {
    // Spawn multiple asynchronous tasks concurrently
    let tasks = vec![
        tokio::spawn(task_with_delay(1)),
        tokio::spawn(task_with_delay(2)),
        tokio::spawn(task_with_delay(3)),
    ];

    // Wait for all tasks to complete
    for task in tasks {
        task.await.expect("Failed to await task");
    }

    println!("All tasks have completed");
}
/*
Op=>
warning: `Async_tasks` (bin "Async_tasks") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 22.67s
     Running `target/debug/Async_tasks`
Task 1 started
Task 2 started
Task 3 started
Task 2 completed
Task 3 completed
Task 1 completed
All tasks have completed
*/