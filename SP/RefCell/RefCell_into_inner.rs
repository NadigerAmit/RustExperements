use std::cell::RefCell;
fn main() {
    let value = 101;
    let ref_cell = RefCell::new(value);

    // Get the inner value from the RefCell
    let inner_value = ref_cell.into_inner();
    println!("Inner value: {}", inner_value);
}
/*
Op => 
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/SP$ ./RefCell_into_inner
Inner value: 101
*/


