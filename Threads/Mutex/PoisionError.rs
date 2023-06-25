use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let mutex = Arc::new(Mutex::new(0));

    // Spawn a thread that panics while holding the lock
    let thread_mutex = Arc::clone(&mutex);
    let _ = thread::spawn(move || {
        let mut lock = thread_mutex.lock().unwrap();
        *lock += 1;
        panic!("OMG, something went wrong!");
    }).join();

    // Attempt to acquire the lock
    match mutex.lock() {
        Ok(mut lock) => {
            *lock += 1;
            println!("Value: {}", *lock);
        }
        Err(poisoned) => {
			
            println!("Mutex is poisoned: {:?}", poisoned);
			 // Handle the poisoned lock case
			let lock = poisoned.into_inner();
            println!("Mutex was poisoned: Retived valkue using into_inner() = {:?}", lock);
        }
    };
}
/*
Op => 
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/DS/Threads/Mutex$ ./PoisionError
thread '<unnamed>' panicked at 'OMG, something went wrong!', PoisionError.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Mutex is poisoned: PoisonError { .. }
Mutex was poisoned: Retived valkue using into_inner() = 1
*/
