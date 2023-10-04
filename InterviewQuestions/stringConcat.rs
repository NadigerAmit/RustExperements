use std::io;
fn main() {
	let mut str_data1:String = String::new();
	println!("Enter the string1 for concat");
	std::io::stdin().read_line(&mut str_data1).expect("Invalid input data");
	let mut str_data2:String = String::new();
	println!("Enter the string1 for concat");
	std::io::stdin().read_line(&mut str_data2).expect("Invalid input data");
	str_data1.push_str(str_data2.as_str());
	println!("Concaatenated string = {}",str_data1);
	
	let mut str1:String = String::from("Jai Shree Ram");
	let mut str2:String = String::from(" Jai Bajrang bali ");
	str1.push_str(str2.as_str());
	println!("Concaatenated string = {}",str1);
}
