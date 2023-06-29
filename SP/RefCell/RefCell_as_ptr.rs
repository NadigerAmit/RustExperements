use std::cell::RefCell;
fn main() {
	let value = 101;
    let ref_cell = RefCell::new(value);

    // Get a raw pointer to the value inside the RefCell
    let raw_ptr = ref_cell.as_ptr();
    println!("Raw pointer address: {:?}", raw_ptr);
	println!("Raw pointer value: {:?}", unsafe{*raw_ptr});
}

