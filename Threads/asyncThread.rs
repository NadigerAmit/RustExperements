/*
Demo of creating the async thread
*/
use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();

    // Spawn a thread to send values
    let handle = thread::spawn(move || {
        sender.send(42).unwrap();
        sender.send(99).unwrap();
        sender.send(123).unwrap();
    });

    // Receive values in the main thread
	while let Ok(msg) = receiver.recv() {
		println!("Received: {:?}", msg);
	}
	
    let res = handle.join();
    match res	{
		Ok(val) => println!("Success in joining thread "),
		Err(_msg) => {
			println!("Eroor in joining thread");
		},
	}
}
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/DS/Threads$ ./asyncThread
Received: 42
Received: 99
Received: 123
Success in joining thread
*/
