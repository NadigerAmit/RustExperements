use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

    let borrowed = data.borrow(); // Immutable borrow
    println!("Value: {}", *borrowed);

    // Attempt to mutate the value
    // This will panic at runtime if there are active borrows
    *data.borrow_mut() += 1;
    println!("Updated value: {}", *borrowed); 
}
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/SP$ ./RefCell
Value: 5
thread 'main' panicked at 'already borrowed: BorrowMutError', RefCell.rs:11:11
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/