use std::borrow::Cow;

fn main() {
    let borrowed_data: Cow<str> = Cow::Borrowed("Jai Shree Ram!");
    let owned_data: Cow<str> = Cow::Owned(String::from("Jai Bajrang bali!"));

    let borrowed_ref: &str = borrowed_data.as_ref();
    let owned_ref: &str = owned_data.as_ref();

    println!("Borrowed reference: {}", borrowed_ref);
    println!("Owned reference: {}", owned_ref);
}
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/SP/Cow$ ./as_ref
Borrowed reference: Jai Shree Ram!
Owned reference: Jai Bajrang bali!
*/
