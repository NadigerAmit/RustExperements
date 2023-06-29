use std::cell::Cell;

fn main() {
    let data = Cell::new(5);

    let borrowed = data.get(); // Immutable borrow
    println!("Value: {}", borrowed);

    // Attempt to mutate the value
    // This will not panic at runtime even if there are active borrows
    data.set(101);
    println!("Updated value: {}", data.get()); 
}
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/SP$ ./CellUsage
Value: 5
Updated value: 101
*/