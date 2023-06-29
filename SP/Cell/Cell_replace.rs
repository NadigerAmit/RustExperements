use std::cell::Cell;

fn main() {
    let value = Cell::new(100);
	let old_value = value.replace(101);
    let replaced_value = value.get();
	println!("Old value: {}", old_value);
    println!("Replaced value: {}", replaced_value);
}
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/SP$ ./Cell_replace
Old value: 100
Replaced value: 101
*/