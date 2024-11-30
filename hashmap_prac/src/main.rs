use std::collections::HashMap;
use std::collections::hash_map::Entry;
fn main() {

    // Example 1: Create a basket of items with their counts.
    // the key will be of the type String and the value will be of the type i32
    let mut basket:HashMap<String, i32>= HashMap::new();

    // just checking the capacity of the hashmap
    println!("basket.capacity() = {}",basket.capacity());   

    // adding a item to the basket using the or_default method.
    // entry method returns a Entry enum which has three variants: Occupied, Vacant and OrDefault
    // occupied is used when the key already exists and vacant is used when the key does not exist.
    // or_default is used when the key does not exist and we want to insert a default value into the hashmap.
    let value = basket.entry(String::from("apple")).or_default();
    println!("value = {:?}",value);
    // printing the basket after the 1st insertion
    println!("basket = {:?}",basket);

    // adding a item to the basket using the or_insert method.
    // or_insert is used when the key does not exist and we want to insert a value into the hashmap.
    // if the key already exists, it will not insert the value and will return the existing value.
    
    // Using entry() to get a mutable reference to the value associated with "banana"
    // or_insert(10) will insert 10 if "banana" doesn't exist, otherwise returns reference to existing value
    // value2 will be a mutable reference (&mut i32) to either the new or existing value
    let value2 = basket.entry(String::from("banana"));
    match value2 {
        Entry::Occupied(mut entry) => {
            *entry.get_mut() += 1;
        }
        Entry::Vacant(entry) => {
            entry.insert(1);
        }
    }
    // printing the basket after the 2nd insertion
    println!("basket = {:?}",basket);


    // Example 2: Counting the frequency of words in a string
    let mut word_counts: HashMap<String, i32> = HashMap::new();
    for word in "hello world hello".split_whitespace() {
        *word_counts.entry(word.to_string()).or_default() += 1;
    }
    println!("word_counts = {:?}",word_counts);
}
