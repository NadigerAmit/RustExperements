use std::sync::mpsc;
use std::thread;

fn main() {
    // Create a synchronous channel with a capacity of 1
    let (tx, rx) = mpsc::sync_channel(1);
	//let (tx, rx) = mpsc::channel();

    // Spawn a thread to send a message
	let tx1 = tx.clone();
    thread::spawn(move || {
        let message = String::from("Jai from thread 1!");
        tx1.send(message).expect("Failed to send message");
        println!("Sender: Message sent from thread 1!");
    });

    let tx2 = tx.clone();
	thread::spawn(move || {
        let message = String::from("Shree Thread 2!");
        tx2.send(message).expect("Failed to send message");
        println!("Sender: Message sent");
    });
	
    let tx3 = tx.clone();
	thread::spawn(move || {
        let message = String::from("Ram Thread 3!");
        tx3.send(message).expect("Failed to send message");
        println!("Sender: Message sent from thread 2!");
    });
	
	 let tx4 = tx.clone();
	thread::spawn(move || {
        let message = String::from("jai Bajarang Bali Thread 4!");
        tx4.send(message).expect("Failed to send message");
        println!("Sender: Message sent from thread 3!");
    });
     
	drop(tx);
    // Receive the message from the channel
	while let Ok(msg) = rx.recv() {
		println!("Receiver: Received message: {}", msg);
	}  

}