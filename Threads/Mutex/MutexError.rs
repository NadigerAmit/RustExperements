use std::sync::Mutex;

fn main() {
    let data = Mutex::new(0);

    let lock_result = data.lock();
    match lock_result {
        Ok(mut value) => {
            *value += 1;
			println!("Value = {}",value);
        }
        Err(err) => {
            eprintln!("Failed to acquire lock: {}", err);
        }
    }
	
}