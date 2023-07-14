use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();

    // Inserting values using the Entry enum variants
    map.insert("Key1".to_string(), 10);
    map.insert("Key2".to_string(), 20);

    match map.entry("Key1".to_string()) {
        std::collections::hash_map::Entry::Occupied(o) => {
            println!("Occupied entry for key 'Key1'");
            println!("Existing value: {}", o.get());
           // o.and_modify(|value| *value += 5);
            println!("Modified value: {}", o.get());
        }
        std::collections::hash_map::Entry::Vacant(v) => {
            println!("Vacant entry for key 'Key1'");
            v.insert(15);
          //  println!("Inserted value: {}", v.get());
        }
    }

    match map.entry("Key3".to_string()) {
        std::collections::hash_map::Entry::Occupied(o) => {
            println!("Occupied entry for key 'Key3'");
            println!("Existing value: {}", o.get());
          //  o.and_modify(|value| *value += 5);
          //  println!("Modified value: {}", o.get());
        }
        std::collections::hash_map::Entry::Vacant(v) => {
            println!("Vacant entry for key 'Key3'");
            v.insert(30);
           // println!("Inserted value: {}", v.get());
        }
    }

    // Iterating over the entries
    for (key, value) in &map {
        println!("Key: {}, Value: {}", key, value);
    }
}
/*
Op -> 
amit@DESKTOP-9LTOFUP:~/OmPracticeRust/DS/Map$ ./Entry
Occupied entry for key 'Key1'
Existing value: 10
Modified value: 10
Vacant entry for key 'Key3'
Key: Key2, Value: 20
Key: Key3, Value: 30
Key: Key1, Value: 10
*/
/*
In the example, we create a HashMap called map and insert values for keys "Key1" and "Key2".

We then demonstrate the usage of the Entry enum and its variants:

For the entry with key "Key1", we use Occupied to check if it exists. If it does, we print the existing value, modify it using and_modify(), and print the modified value. 
If it doesn't exist, we use Vacant to indicate that it's not present, insert a new value of 15, and print the inserted value.
For the entry with key "Key3", we follow the same pattern as above. Since it doesn't exist, we use Vacant to indicate that it's not present, insert a new value of 30, and print the inserted value.
Lastly, we iterate over the entries in the map and print each key-value pair.

The output of running this code will demonstrate the behavior of the Entry enum's variants, showing whether an entry is occupied or vacant, and performing appropriate operations based on the case.
*/