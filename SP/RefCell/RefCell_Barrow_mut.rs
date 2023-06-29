use std::cell::RefCell;

fn main() {
    let value = 101;
    let ref_cell = RefCell::new(value);

    // Borrow the value mutably
    let mut borrowed_mut_value = ref_cell.borrow_mut();
    *borrowed_mut_value += 10;
    println!("Modified value: {}", *borrowed_mut_value);
}

/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/SP$ ./RefCell_Barrow_mut
Modified value: 111
*/