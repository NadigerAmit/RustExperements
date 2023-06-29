use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

 // Commented below 2 line for successfull running . i.e avoiding the immutable barrow
    //let borrowed = data.borrow(); // Immutable borrow
    //println!("Value: {}", *borrowed);

    // Attempt to mutate the value
    // This will panic at runtime if there are active borrows
    *data.borrow_mut() += 1;
    println!("Updated value: {}", *data.borrow()); 
}
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/SP$ ./RefCellSuccesDemo
Updated value: 6
*/