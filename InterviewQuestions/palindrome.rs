fn is_string_palindrome_efficient(input_str:&str)->bool {
	let rev_str:String = input_str.to_lowercase().chars().rev().collect();
	let isPalindrom = (input_str.to_lowercase() == rev_str);
	isPalindrom
}

use std::io;
fn main() {
	let mut str_data:String = String::new();
	println!("Enter the string to check for palindrome");
	std::io::stdin().read_line(&mut str_data).expect("Invalid input data");
	
	//println!("Is Gadag is palindrom {}",str_data,is_string_palindrome_efficient("Gadag"));
	println!("Is string {} palindrom {}",str_data,is_string_palindrome(&str_data.trim()));
}

fn is_string_palindrome(str_data:&str)->bool {
	let vec_str:Vec<char> = str_data.to_lowercase().chars().collect();
	let mut len = str_data.len();
	for i in 0..str_data.len() {
		if vec_str[i] != vec_str[len-1] {
			return false;
		}
		len -=1;
	}
	return true;
}
