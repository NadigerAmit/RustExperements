use std::borrow::ToOwned;

fn main() {
    let borrowed_str: &str = "Jai Shree Ram!";
    let mut owned_string: String = borrowed_str.to_owned();
	owned_string.push_str("Jai Bajrang bali");
	
	println!("Barrowed string: {}",borrowed_str);

    println!("Owned string: {}", owned_string);
}
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust$ ./To_Owned_demo
Barrowed string: Jai Shree Ram!
Owned string: Jai Shree Ram!Jai Bajrang bali
*/