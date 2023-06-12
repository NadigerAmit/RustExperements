use std::thread;

struct LargeData {
    // large data structure
    data: Vec<u8>,
}

fn process_large_data(data: Box<LargeData>) {
    // Process the large data here
    // ...

    // Example: Print the length of the data
    println!("Length of data: {}", data.data.len());
}

fn main() {
    let data = Box::new(LargeData {
        // Initialize the large data structure
        data: vec![1, 2, 3, 4, 5],
    });

    // Spawn a new thread and transfer ownership of `data`
    let handle = thread::spawn(move || {
        process_large_data(data);
    });

    // Wait for the spawned thread to finish
    handle.join().unwrap();
}
/*
Length of data: 5
*/
