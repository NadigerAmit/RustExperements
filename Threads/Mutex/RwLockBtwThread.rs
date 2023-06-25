use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

fn main() {
    // Create a shared data structure wrapped in an `Arc`
    let shared_data = Arc::new(RwLock::new(0));
	let shared_data_clone = Arc::clone(&shared_data);
	
	    // Spawn a thread to write to the shared data
    thread::spawn(move || {
        // Acquire an exclusive lock to write the data
        let mut writer = shared_data_clone.write().unwrap();
		std::thread::sleep(std::time::Duration::from_secs(1));
        *writer = 10;
        println!("Write in 1st write thread: {}", *writer);
        // Lock is automatically released when `writer` goes out of scope
    });

    // Spawn multiple threads to read from the shared data concurrently
    for _ in 0..5 {
        let data = Arc::clone(&shared_data);
        thread::spawn(move || {
            // Acquire a shared lock to read the data
            let reader = data.read().unwrap();
            println!("Read: {}", *reader);
            // Lock is automatically released when `reader` goes out of scope
        });
    }

    // Spawn a thread to write to the shared data
    thread::spawn(move || {
        // Acquire an exclusive lock to write the data
        let mut writer = shared_data.write().unwrap();
        *writer = 101;
        println!("Write in second write thread: {}", *writer);
        // Lock is automatically released when `writer` goes out of scope
    });

    // Wait for all threads to finish
    thread::sleep(std::time::Duration::from_secs(2));
}
/*
Op => 

amit@DESKTOP-9LTOFUP:~/OmPracticeRust/DS/Threads/Mutex$ rustc RwLockBtwThread.rs
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/DS/Threads/Mutex$ ./RwLockBtwThread
Read: 0
Read: 0
Write in 1st write thread: 10
Write in second write thread: 101
Read: 101
Read: 101
Read: 101

amit@DESKTOP-9LTOFUP:~/OmPracticeRust/DS/Threads/Mutex$ ./RwLockBtwThread
Write in 1st write thread: 10
Write in second write thread: 101
Read: 101
Read: 101
Read: 101
Read: 101
Read: 101

*/