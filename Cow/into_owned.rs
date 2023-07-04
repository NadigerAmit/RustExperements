use std::borrow::Cow;

fn main() {
    let borrowed_data: Cow<str> = Cow::Borrowed("Krishanaya vasudevaya , 
	        Hareye parmatmne!");

    // move occurs because `borrowed_data` has type `Cow<'_, str>`,
	let owned_data: String = borrowed_data.into_owned(); 

    println!("Owned data: {}", owned_data);
}
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/SP/Cow$ ./into_owned
Owned data: Krishanaya vasudevaya , Hareye parmatmne!
*/
