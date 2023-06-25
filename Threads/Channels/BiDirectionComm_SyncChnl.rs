/*
Bi directional communication using 2 seperate sync channel.
The communication between thread 1 and 2 are synced .
1st thread1 sends msg
Then thread 2 receives
Then thread2 send message to thread1
Then thread 1 receives the message from Thread2
*/
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx1, rx1) = mpsc::sync_channel(1); // Channel 1 for communication from sender1 to receiver1
    let (tx2, rx2) = mpsc::sync_channel(1); // Channel 2 for communication from sender2 to receiver2

    // Spawn two threads for the senders
    let handle1 = thread::spawn(move || {
		let s = String::from("Hello(Jai ShreeRam)");
        tx1.send(s.clone()).unwrap();
		println!("Thread 1 sent the message to thrd 2 => {}: ",s);
        let received:String = rx2.recv().unwrap();
        println!("Thread 1 received: => {}", received);
    });

    let handle2 = thread::spawn(move || {
		let s = String::from("Hello(Jai Bajrang bali)");
        let received = rx1.recv().unwrap();
		//let received: String = rx1.recv().unwrap();
        println!("Thread 2 received: => {}", received);
		//tx1.send("Message from sender1, Hello(Jai ShreeRam   )".to_String()).unwrap();
		tx2.send(s.clone()).unwrap();
		println!("Thread 2 sent the message to thrd 1 =>{}: ",s);
    });

    // Wait for the sender threads to finish
    handle1.join().unwrap();
    handle2.join().unwrap();
}
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/DS/Threads$ ./BiDirectionComm_SyncChnl
Thread 1 sent the message to thrd 2 => Hello(Jai ShreeRam):
Thread 2 received: => Hello(Jai ShreeRam)
Thread 2 sent the message to thrd 1 =>Hello(Jai Bajrang bali):
Thread 1 received: => Hello(Jai Bajrang bali)
*/
