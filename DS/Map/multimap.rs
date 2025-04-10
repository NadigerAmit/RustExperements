/*
Easy to understand multi map implementation  
*/
use std::collections::HashMap;
fn main() {
    let mut mmap:HashMap<i32,Vec<String>> = HashMap::new();
    let v = vec![
        (1, "Jai"),
        (1, "Shree"),
        (1, "Ram"),
        (2, "Om"),
        (2, "Namah"),
        (2, "Shivaya"),
        (0, "JaiBajarangBali"),
    ];
   // Easy to understand way to implement 
    for i in v {
        if mmap.contains_key(&i.0) {
            mmap.get_mut(&i.0).unwrap().push(i.1.to_string());
        } else {
            mmap.insert(i.0,vec![String::from(i.1)]);
        }
    }
/*
  //  Standard way to implement 
    for (key, value) in v {
        mmap.entry(key)
        .or_insert_with(Vec::new)
        .push(value.to_string());
    }
	*/
    println!("{:?}",mmap);
}
/*
{1: ["Jai", "Shree", "Ram"], 2: ["Om", "Namah", "Shivaya"], 0: ["JaiBajarangBali"]}
*/
