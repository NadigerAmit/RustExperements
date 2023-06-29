use std::cell::RefCell;

fn main() {
    let value = 101;
    let ref_cell = RefCell::new(value);

    // Replace the value inside the RefCell
    let new_value = 100;
    let oldValue = ref_cell.replace(new_value);
	let replaced_value = ref_cell.borrow();
    println!("Old value: {}", oldValue);
	println!("New value: {}", replaced_value);
}
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/SP$ ./RefCell_replace
Old value: 101
New value: 100
*/
