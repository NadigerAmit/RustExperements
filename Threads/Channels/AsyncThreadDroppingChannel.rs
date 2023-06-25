
/*
Demonstrate the dorpping the sender to signal the receiver that no message will be sent
*/
use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();

    // Spawn a thread to receive messages
    let receiver_thread = thread::spawn(move || {
        loop {
            match receiver.recv() {
                Ok(message) => println!("Received: {}", message),
                Err(_) => {
                    println!("Sender dropped, exiting receiver loop");
                    break;
                }
            }
        }
    });

    // Send some messages
    sender.send("Jai Shree Ram").unwrap();
    sender.send("Jai Bajarang Bali").unwrap();

    // It's important to drop the sender to signal the end of messages
    drop(sender);

    // Wait for the receiver thread to finish
    receiver_thread.join().unwrap();
}
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/DS/Threads$ ./AsyncThreadDroppingChannel
Received: Jai Shree Ram
Received: Jai Bajarang Bali
Sender dropped, exiting receiver loop
*/