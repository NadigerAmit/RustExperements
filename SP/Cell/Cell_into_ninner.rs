use std::cell::Cell;

fn main() {
    let value = Cell::new(101);
    let inner_value = value.into_inner();
    println!("Inner value: {}", inner_value);
}
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/SP$ ./Cell_into_ninner
Inner value: 10
*/
