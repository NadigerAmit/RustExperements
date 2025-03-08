// Online Rust compiler to run Rust program online
// Print "Try programiz.pro" message
use std::collections::HashSet;
fn main() {
    let v = vec![9,3,9,3,9,7,9];
    let mut s:HashSet<i32> = HashSet::new();
    for i in v.iter() {
        if s.contains(i) {
            s.remove(i);  
        } else {
            s.insert(*i);
        }
    }
    println!("OddOccurrencesInArray - {}",s.iter().next().unwrap());
    
}