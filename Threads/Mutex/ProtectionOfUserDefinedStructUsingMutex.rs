use std::sync::Mutex;
use std::sync::Arc;

// User-defined struct
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn decrement(&mut self) {
        self.count -= 1;
    }

    fn get_count(&self) -> u32 {
        self.count
    }
}

fn main() {
    // Create a Mutex to protect the Counter struct
   // let counter_mutex = Mutex::new(Counter::new());
	let counter_mutex = Arc::new(Mutex::new(Counter::new()));

    // Spawn multiple threads to access and modify the counter concurrently
    let mut handles = vec![];
	println!("Increamenting the counter ");

    for _ in 0..5 {
		let counter_mutex_clone = Arc::clone(&counter_mutex);
        let handle = std::thread::spawn(move || {
            // Acquire the lock to access the Counter
            let mut counter = counter_mutex_clone.lock().unwrap();

            // Modify the counter
            counter.increment();
           // counter.decrement();

            // Print the current count
            println!("Count: {}", counter.get_count());
        });

        handles.push(handle);
    }
	println!("Decreamenting the counter ");
	for _ in 0..5 {
		let counter_mutex_clone = Arc::clone(&counter_mutex);
        let handle = std::thread::spawn(move || {
            // Acquire the lock to access the Counter
            let mut counter = counter_mutex_clone.lock().unwrap();

            // Modify the counter
            //counter.increment();
			counter.decrement();

            // Print the current count
            println!("Count: {}", counter.get_count());
        });
		
        handles.push(handle);
    }
    
	
    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
	let mut counter = counter_mutex.lock().unwrap();
    println!("Final Count of counter = {}",counter.get_count());
}
/*

amit@DESKTOP-9LTOFUP:~/OmPracticeRust/DS/Threads/Mutex$ ./ProtectionOfUserDefinedStructUsingMutex
Increamenting the counter
Count: 1
Count: 2
Decreamenting the counter
Count: 3
Count: 4
Count: 5
Count: 4
Count: 3
Count: 2
Count: 1
Count: 0
Final Count of counter = 0
*/
