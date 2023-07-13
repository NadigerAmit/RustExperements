use std::collections::HashMap;

fn main() {
    // Create an empty multi-map
    let mut multi_map: HashMap<String, Vec<String>> = HashMap::new();

    // Insert values into the multi-map
    multi_map.entry("Key1".to_string()).or_insert(Vec::new()).push("Value1".to_string());
    multi_map.entry("Key1".to_string()).or_insert(Vec::new()).push("Value2".to_string());
    multi_map.entry("Key2".to_string()).or_insert(Vec::new()).push("Value3".to_string());
    multi_map.entry("Key2".to_string()).or_insert(Vec::new()).push("Value4".to_string());

    // Access values from the multi-map
    if let Some(values) = multi_map.get("Key1") {
        for value in values {
            println!("Value: {}", value);
        }
    }
    if let Some(values) = multi_map.get("Key2") {
        for value in values {
            println!("Value: {}", value);
        }
    }
}
/*
Value: Value1
Value: Value2
Value: Value3
Value: Value4

*/