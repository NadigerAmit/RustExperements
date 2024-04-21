/*
// Most concise way :
fn main() {
	let str = String::from("JaiShreeRam");
	let rev_str:String = str.chars().rev().collect();
	println!("str  {}",str);
	println!("rev_str = {}",rev_str);
}
*/

// Below is the traditional way 
use std::io;

fn main() {
    println!("Enter the string of u r choice");
    let mut string_data:String = String::new();
    std::io::stdin().read_line(&mut string_data).expect("Error in string input");
    let mut str_vec:Vec<char> = string_data.chars().collect();
    
    reverse_vec(&mut str_vec);
    let reversed_string:String = str_vec.into_iter().collect();
    println!("Reversed String =  {}",reversed_string);
}

fn reverse_vec(string_vec:&mut Vec<char>)  {
    let mut len = string_vec.len();
    for i in 0..string_vec.len() {
        if i >= len-1 {
        	break;
        }
    	let temp = string_vec[i];
    	string_vec[i] = string_vec[len-1];
    	string_vec[len-1] = temp;
    	len -=1; 
    	
    }
}

