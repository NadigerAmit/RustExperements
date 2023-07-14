use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();


    // Inserting or modifying values using the Entry enum
    map.entry("Key".to_string()).or_insert(10);
    map.entry("AnotherKey".to_string()).or_insert(20);
	map.entry("Key".to_string()).or_insert(100);
	
	println!("Map entries are below ");
	for (k,v) in &map {
		println!("k = {} -> v = {}",k,v);
	}

    match map.entry("Key".to_string()) {
        std::collections::hash_map::Entry::Occupied(mut val) => {
            println!("Existing value: {}", val.get());
            val.insert(*val.get() + 5);
        }
        std::collections::hash_map::Entry::Vacant(v) => {
            println!("Entry not found for key: {}", v.key());
        }
    }

    // Accessing the modified value
    let modified_value = map.get("Key");
    println!("Modified value: {:?}", modified_value);  // Output: Some(15)
}
/*
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/DS/Map$ ./EntryEnumOfMap
Existing value: 10
Modified value: Some(15)
*/