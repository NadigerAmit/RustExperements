use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(0);

    // Acquire a read lock
    {
        let reader = lock.read().unwrap();
        println!("Read data: {}", *reader);
        // The read lock is automatically released at the end of this block
    }

    // Acquire a write lock
    {
        let mut writer = lock.write().unwrap();
        *writer += 10;
        println!("Modified data: {}", *writer);
        // The write lock is automatically released at the end of this block
    }
}
/*
Op => 
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/DS/Threads/Mutex$ ./RwLockGuard
Read data: 0
Modified data: 10
*/