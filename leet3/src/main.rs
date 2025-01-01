use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    if s.len() < 1 {
        return 0;
    }
    // To find longest substring in the given string
    // first pointer, i = 0
    // second pointer, j = 0
    // start moving the second pointer to the right indices of the string, as long as
    // there is no repeating characters
    let mut l = 0;
    let mut r = 0;
    let mut map: HashMap<char, i32> = HashMap::new();
    let mut ans = 0;
    let char_vec: Vec<char> = s.chars().collect();
    // ToDo: condition to be added later on
    while r < s.len() {
        let curr_char = char_vec[r];

        // now check if the curr_char is already in the HashMap
        if !map.contains_key(&curr_char) {
            // if above statement is true then, add the key to the map
            map.insert(curr_char, 1);
            r += 1;
            ans = ans.max(r - l);
        } else {
            // duplicate character found
            // in this case, do not touch the r pointer, but remove the item on the l index and then
            // move the l index pointer to the right. (it is not technically a pointer)

            // in rust, use the get_mut() instead of get() since we need to modify the value of the
            // item the key points to. So we need a mutable reference.
            let left_char = char_vec[l];
            if let Some(count_val) = map.get_mut(&left_char) {
                *count_val -= 1;

                if *count_val == 0 {
                    map.remove(&left_char);
                }
            }
            l += 1;
        }
        // because r index is exclusive of the current window as it points to the next index char.
    }
    ans as i32
}

fn main() {
    println!("{}", length_of_longest_substring("abcabcbb".to_string()));
    println!("{}", length_of_longest_substring("pwwkew".to_string()));
    println!("{}", length_of_longest_substring("dvdf".to_string()));
    println!("{}", length_of_longest_substring(" ".to_string()));
    println!("{}", length_of_longest_substring("tmmzuxt".to_string()));
}