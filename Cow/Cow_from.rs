use std::borrow::Cow;

fn main() {
    let borrowed_data: Cow<str> = Cow::from("Jai Shree Ram!");
    let owned_data: Cow<str> = Cow::from(String::from("Jai Bajarang bali!"));

    println!("Borrowed data: {:?}", borrowed_data);
    println!("Owned data: {:?}", owned_data);
}
