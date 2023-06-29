use std::cell::RefCell;

fn main() {
    let mut value = 101;
    let mut ref_cell = RefCell::new(&mut value);

    // Get a mutable reference to the value inside the RefCell
    let mutable_value = ref_cell.get_mut();
    **mutable_value += 10;
    println!("Modified value: {}", *mutable_value);
}
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/SP$ ./RefCell_getMut
Modified value: 111
*/
