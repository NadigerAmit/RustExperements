/*
Objective ' Reverse the string in long sentense 
Ex: India is great country to live 
*/

use std::io;
fn reverse_string(input:&mut String) {
    let vec_str:Vec<String> = input.trim().split(" ").map(|x|x.to_string()).collect();
    let mut result_string:String = String::new();
    input.clear();
    for i in vec_str {
        let rtev_str:String = i.chars().rev().collect();
        input.push_str(&rtev_str);
        input.push_str(" ")
       // result_string += &rtev_str ;
       // result_string += " ";
    }
}
fn main() {

    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    reverse_string(&mut input);
    println!("Reversed string {}",input); 
}