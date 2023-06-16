/*
Demonstrate the multiple producer and snigle sender.
cloning the sender 
droping the unmoved sender to indicate the receiver no more message will be sent.
*/
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel(); // Unbounded channel
    let tx1 = tx.clone();
    let handle1 = thread::spawn(move || {
        println!("Hello from thread1");
        let message: String = String::from("Jai-Thread1");
        tx1.send(message).unwrap();
		// At the end of this block , tx1 is automatically dropped 
		// since tx1 is shared with this thread via move. 
    });

    let tx2 = tx.clone();
    let handle2 = thread::spawn(move || {
        println!("Hello from thread2");
        let message: String = String::from("Shree-Thread2");
        tx2.send(message).unwrap();
        // At the end of this block , tx2 is automatically dropped 
		// since tx2 is shared with this thread via move. 
    });

    let tx3 = tx.clone();
    let handle3 = thread::spawn(move || {
        println!("Hello from thread3");
        let message: String = String::from("Ram-Thread3");
        tx3.send(message).unwrap();
        // At the end of this block , tx3 is automatically dropped 
		// since tx3 is shared with this thread via move. 
    });

    println!("Om from-Thread1");
     // We need to tx as it is still in scope , if we dont drop the below while loop 
	 //looking for message will be waiting indefinatly to look for messages from tx 
    drop(tx); // Drop the sender to close the channel

    while let Ok(message) = rx.recv() {
        println!("Received from Thread: {}", message);
    }

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
} 
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/DS/Threads$ ./AsyncThreadWithMultipleProdeucer
Hello from thread1
Hello from thread2
Om from-Thread1
Hello from thread3
Received from Thread: Jai-Thread1
Received from Thread: Shree-Thread2
Received from Thread: Ram-Thread3
/*
In the above code the send method is called on each tx object to send the respective messages through the channel. The unwrap method is used to handle any potential errors that may occur during sending.

After sending all the messages, the tx object in the main thread is dropped using drop(tx). This is necessary to close the channel and signal to the receiving end (rx) that no more messages will be sent.

No need to drop tx1,tx2,tx3 since those cloned channels are moved to thread 1,2,3 respectively via move construct . Hence they are automatically dropped once threads are completed. i.e thread block is completed. 

But tx is not moved to any threads , hence tx need to dropped explicitly.

The while loop with rx.recv() will now exit once all messages have been received, and the program will complete successfully. 
*/