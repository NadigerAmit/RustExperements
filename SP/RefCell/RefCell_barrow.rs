use std::cell::RefCell;

fn main() {
    let value = 101;
    let ref_cell = RefCell::new(value);

    // Borrow the value immutably
    let borrowed_value = ref_cell.borrow();
    println!("Borrowed value: {}", *borrowed_value);

}


