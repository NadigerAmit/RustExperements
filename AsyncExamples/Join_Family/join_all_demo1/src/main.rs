use futures::future::join_all;
use tokio::time::{sleep, Duration};

async fn expensive_computation(data: i32,task_id:i32,delay_sec:u8) -> i32 {
    // Simulate a time-consuming computation
    println!("Task_id = {} <=> delay_sec = {}",task_id,delay_sec);
    sleep(Duration::from_secs(delay_sec.into())).await;
    data * 2
}

#[tokio::main]
async fn main() {
    let data = vec![1, 2, 3, 4, 5,6,7,8,9];
    let results = join_all(data.iter().map(|&d| expensive_computation(d,d,(10-d).try_into().unwrap()))).await;

    println!("Results: {:?}", results);

    for (index, result) in results.into_iter().enumerate() {
        println!("Result {}: {}", index, result);
    }
}
/*
Op => 
   Running `target/debug/join_all_demo1`
Results: [2, 4, 6, 8, 10, 12, 14, 16, 18, 200]
Result 0: 2
Result 1: 4
Result 2: 6
Result 3: 8
Result 4: 10
Result 5: 12
Result 6: 14
Result 7: 16
Result 8: 18
Result 9: 200
*/