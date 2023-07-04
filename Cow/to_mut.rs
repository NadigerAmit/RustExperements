use std::borrow::Cow;

fn main() {
    let mut borrowed_data: Cow<str> = Cow::Borrowed("Om Namah Shivaya!");

    let mut mutable_data = borrowed_data.to_mut();
    mutable_data.make_ascii_uppercase();

    println!("Mutated data: {}", mutable_data);
}
