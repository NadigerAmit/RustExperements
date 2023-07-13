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
/*
In this example, we create a HashMap called multi_map, where the keys are of type String, and the values are vectors of type Vec<String>.
The multi_map represents a multi-map where each key can have multiple values associated with it.

To insert values into the multi-map, we use the entry() method of the HashMap. 
The entry() method returns an Entry enum, which allows us to insert values if the key doesn't exist or access the existing values if the key already exists.
We use the or_insert() method to either insert an empty vector (Vec::new()) or retrieve the existing vector associated with the key, and then we push values into the vector.

To access the values from the multi-map, we use the get() method of the HashMap. 
It returns an Option containing a reference to the value associated with the key, which is a vector in this case.
We can then iterate over the values and perform any desired operations.

Note that in this example, we assume that each key in the multi-map can have multiple values associated with it. 
If you need more advanced multi-map functionality or want to use a dedicated crate, you can explore third-party libraries like multimap or indexmap that provide additional features for working with multi-maps.

*/
