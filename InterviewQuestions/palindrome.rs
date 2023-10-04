use std::io;
fn main() {
	let mut str_data:String = String::new();
	println!("Enter the string to check for palindrome");
	std::io::stdin().read_line(&mut str_data).expect("Invalid input data");
	
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
