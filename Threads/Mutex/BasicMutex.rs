
/*
To demo basic use of Mutex 
*/
use std::sync::{Arc, Mutex};
use std::thread;
use std::process::id;

fn main() {
    // Create a shared mutable data protected by a Mutex
    let data = Arc::new(Mutex::new(0));

    // Spawn multiple threads to increment the data concurrently
    for i in 0..10 {
        let data = Arc::clone(&data);
		let builder = thread::Builder::new()
                               .name("my_thread".to_string())
                              .stack_size(32 * 1024);
    let child = builder.spawn(move || {
    //    std::thread::spawn(move || {
            // Acquire a lock on the Mutex
            let mut lock = data.lock().unwrap();

            // Access the protected data
			std::thread::sleep(std::time::Duration::from_secs(1));
            *lock += 1;
			let thread = thread::current();
			println!("ProcessId = {:?} , ThreadId = {:?},value = {}",id(),thread.id(),*lock);
        });
    }

    // Wait for all threads to finish
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Access the final value of the shared data
    let lock = data.lock().unwrap();
    println!("Final value: {}", *lock);
}
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/DS/Threads$ ./BasicMutex
ProcessId = 768 , ThreadId = ThreadId(2),value = 1
ProcessId = 768 , ThreadId = ThreadId(3),value = 2
ProcessId = 768 , ThreadId = ThreadId(4),value = 3
ProcessId = 768 , ThreadId = ThreadId(5),value = 4
ProcessId = 768 , ThreadId = ThreadId(6),value = 5
ProcessId = 768 , ThreadId = ThreadId(7),value = 6
ProcessId = 768 , ThreadId = ThreadId(8),value = 7
ProcessId = 768 , ThreadId = ThreadId(9),value = 8
ProcessId = 768 , ThreadId = ThreadId(10),value = 9
ProcessId = 768 , ThreadId = ThreadId(11),value = 10
Final value: 10
*/
