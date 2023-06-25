use std::sync::Mutex;

struct Counter {
    value: Mutex<u32>,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            value: Mutex::new(0),
        }
    }

    fn increment(&self) {
        let mut value = self.value.lock().unwrap();
        *value += 1;
    }

    fn get_value(&self) -> u32 {
        *self.value.lock().unwrap()
    }
}

fn main() {
    let counter = Counter::new();

    counter.increment();
    counter.increment();

    let value = counter.get_value();
    println!("Counter value: {}", value);
}