use std::collections::HashMap;

fn main() {
	let sample_string:String = String::from("Shree ram jai ram jai jai ram");
	let map = count_chars_in_String(sample_string.as_str());
	println!("map = {:?}",map);
}

fn count_chars_in_String(data:&str) -> HashMap<char,u32>{
    let mut map :HashMap<char,u32> = std::collections::HashMap::new();
    let vec_char:Vec<char> = data.chars().collect();
    for i in vec_char {
        if let Some(values) = map.get(&i) {
            map.insert(i,values+1)
        } else {
        	map.insert(i,1)
        };
    }
    println!("String received = {}",data);
	map
}
