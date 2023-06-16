use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx1, rx1) = mpsc::channel(); // Channel 1 for communication from sender1 to receiver1
    let (tx2, rx2) = mpsc::channel(); // Channel 2 for communication from sender2 to receiver2

    // Spawn two threads for the senders
    let handle1 = thread::spawn(move || {
        tx1.send("Message from sender1").unwrap();
        let received:String = rx2.recv().unwrap();
        println!("Sender1 received: {}", received);
    });

    let handle2 = thread::spawn(move || {
        //tx2.send("Message from sender2").unwrap();
        let received = rx1.recv().unwrap();
		//let received: String = rx1.recv().unwrap();
        println!("Sender2 received: {}", received);
    });

    // Wait for the sender threads to finish
    handle1.join().unwrap();
    handle2.join().unwrap();
}
