/*
Purpose of this program is demonstrate the difference between sync and async channels.
Espically w.r.t blocking natuture of sync channels and non-blocking nature of async channels.

*/
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Synchronous channel with capacity 1
    let (sync_sender, sync_receiver) = mpsc::sync_channel(1);

    // Asynchronous channel
    let (async_sender, async_receiver) = mpsc::channel();

    // Spawn a thread for synchronous communication
    let sync_thread = thread::spawn(move || {
        sync_sender.send("SyncMsg1 from synchronous channel").unwrap();
        println!("Synchronous message1 sent");
        sync_sender.send("SyncMsg2 from synchronous channel").unwrap();
        println!("Synchronous message2 sent");
		sync_sender.send("SyncMsg3 from synchronous channel").unwrap();
        println!("Synchronous message3 sent");
		sync_sender.send("SyncMsg4 from synchronous channel").unwrap();
        println!("Synchronous message4 sent");
		sync_sender.send("SyncMsg5 from synchronous channel").unwrap();
        println!("Synchronous message5 sent");
    });

    // Spawn a thread for asynchronous communication
    let async_thread = thread::spawn(move || {
        async_sender.send("ASyncMsg1 from asynchronous channel").unwrap();
        println!("Asynchronous message1 sent");
        async_sender.send("ASyncMsg2 from asynchronous channel").unwrap();
        println!("Asynchronous message2 sent");
		async_sender.send("ASyncMsg3 from asynchronous channel").unwrap();
        println!("Asynchronous message3 sent");
		async_sender.send("ASyncMsg4from asynchronous channel").unwrap();
        println!("Asynchronous message4 sent");
		async_sender.send("ASyncMsg5 from asynchronous channel").unwrap();
        println!("Asynchronous message5 sent");
    });

    // Receive messages from synchronous channel

    for message in sync_receiver {
		thread::sleep(Duration::from_secs(1));
        println!("Received from synchronous channel: {}", message);
    }

	    // Receive messages from synchronous channel
    for message in async_receiver {
		thread::sleep(Duration::from_secs(1));
        println!("Received from asynchronous channel: {}", message);
    }
	

    // Wait for the threads to finish
    sync_thread.join().unwrap();
    async_thread.join().unwrap();
}
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/DS/Threads$ ./DiffBwSyncAndAsyncChannel
Synchronous message1 sent
Synchronous message2 sent
Asynchronous message1 sent
Asynchronous message2 sent
Asynchronous message3 sent
Asynchronous message4 sent
Asynchronous message5 sent
Received from synchronous channel: SyncMsg1 from synchronous channel
Synchronous message3 sent
Received from synchronous channel: SyncMsg2 from synchronous channel
Synchronous message4 sent
Received from synchronous channel: SyncMsg3 from synchronous channel
Synchronous message5 sent
Received from synchronous channel: SyncMsg4 from synchronous channel
Received from synchronous channel: SyncMsg5 from synchronous channel
Received from asynchronous channel: ASyncMsg1 from asynchronous channel
Received from asynchronous channel: ASyncMsg2 from asynchronous channel
Received from asynchronous channel: ASyncMsg3 from asynchronous channel
Received from asynchronous channel: ASyncMsg4from asynchronous channel
Received from asynchronous channel: ASyncMsg5 from asynchronous channel
*/
