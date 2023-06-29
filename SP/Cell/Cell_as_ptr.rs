use std::cell::Cell;

fn main() {
    let value = Cell::new(42);
    let raw_ptr = value.as_ptr();
    println!("Raw pointer address : {:?}", raw_ptr);
    println!("Raw pointer value: {:?}", unsafe {*raw_ptr});
}
/*
Op => 
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/SP$ ./Cell_as_ptr
Raw pointer address : 0x7fffe4772a14
Raw pointer value: 42
*/