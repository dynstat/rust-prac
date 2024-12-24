use std::collections::{HashMap, HashSet};

// Problem: LeetCode 2506 - Count Pairs Of Similar Strings
// You are given a 0-indexed string array words.
// Two strings are similar if they consist only of lowercase English letters and are anagrams of each other.
// Return the number of pairs (i, j) such that 0 <= i < j < words.length and words[i] and words[j] are similar.



    // USING BITMASK METHOD
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let n = words.len();
        let mut count = 0;
        let masks: Vec<u32> = words
            .iter()
            .map(|word| word.bytes().fold(0u32, |mask, b| mask | (1 << (b - b'a'))))
            .collect();
        println!("masks -> {:?}", masks);
        for i in 0..n {
            for j in i + 1..n {
                if masks[i] == masks[j] {
                    count += 1;
                }
                // if are_similar_bitmask(&words[i], &words[j]) {
                //     count += 1;
                // }
            }
        }

        count
        
    }






    // USING HASHMAP + COMBINATION FORMULA (for the count of similar words as a pair (nC2))
    pub fn similar_pairs_hashmap(words: Vec<String>) -> i32 {

        // words is a Vec<String>

        let mut hash_map: HashMap<String, i32> = HashMap::new();
        print!("hash_map -> {:?}: \n", hash_map);
        
        for word in words {
            // print!("word -> {:?}: ", word);
            let chars_set = word.chars().collect::<HashSet<_>>();
            // print!("{:?}", chars_set);
            let mut chars_vec: Vec<char> = chars_set.into_iter().collect();
            // print!("{:?}   ", chars_vec);
            chars_vec.sort_unstable();
            let chars_str : String = chars_vec.into_iter().collect();

            hash_map.entry(chars_str).and_modify(|val| *val += 1).or_insert(1);
            // print!("\nhash_map -> {:?}", hash_map);

        }
        // print!("\n\nFINAL hash_map -> {:?}", hash_map);
        
        // now, from the hashmap and its values, we can determine that how many of the items are 
        // similar acc to the condition.
        // Now we can also determine the combinations of them.
        let mut count = 0;
        for freq in hash_map.values(){
            // print!("freq {:?} -> ", freq);
            count += freq * (freq -1) / 2 ;
            // print!("count {:?} -> ", count);

        }
        count
    }





// 1. Bit Masking Method
fn similar_pairs_bitmask(words: Vec<String>) -> i32 {
    let n = words.len();
    let mut count = 0;
    let masks: Vec<u32> = words
        .iter()
        .map(|word| word.bytes().fold(0u32, |mask, b| mask | (1 << (b - b'a'))))
        .collect();
    for i in 0..n {
        for j in i + 1..n {
            if masks[i] == masks[j] {
                count += 1;
            }
            // if are_similar_bitmask(&words[i], &words[j]) {
            //     count += 1;
            // }
        }
    }

    count
}

// fn are_similar_bitmask(s1: &str, s2: &str) -> bool {
//     let mut mask1 = 0;
//     let mut mask2 = 0;

//     for &b in s1.as_bytes() {
//         mask1 |= 1 << (b - b'a');
//     }

//     for &b in s2.as_bytes() {
//         mask2 |= 1 << (b - b'a');
//     }

//     mask1 == mask2
// }

// 2. Hash Map (Character Counts) Method
fn similar_pairs_hashmap2(words: Vec<String>) -> i32 {
    let n = words.len();
    let mut count = 0;

    for i in 0..n {
        for j in i + 1..n {
            if are_similar_hashmap(&words[i], &words[j]) {
                count += 1;
            }
        }
    }

    count
}

fn are_similar_hashmap(s1: &str, s2: &str) -> bool {
    let set1: HashSet<char> = s1.chars().collect();
    let set2: HashSet<char> = s2.chars().collect();
    set1 == set2
}

// 3. Unordered Map (using String as Key for sorted representation)
fn similar_pairs_string_key(words: Vec<String>) -> i32 {
    let mut map: HashMap<String, i32> = HashMap::new();

    // Convert each word to its unique character representation
    for word in words {
        let mut chars: Vec<char> = word.chars().collect::<HashSet<_>>().into_iter().collect();
        chars.sort_unstable();
        let key: String = chars.into_iter().collect();
        *map.entry(key).or_insert(0) += 1;
    }

    // Calculate pairs for each group
    map.values()
        .filter(|count| **count > 1)
        .map(|count| (*count * (*count - 1)) / 2)
        .sum()
}

fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", arr[1]);
    let nums = vec![1, 2, 3, 4, 5];
    println!("{}", nums[1]);
    let words = vec!["aba".to_string(), "aabb".to_string(), "abcd".to_string(), "bac".to_string(), "aabc".to_string(), "cab".to_string()];
    println!("Bitmask: {}", similar_pairs_bitmask(words.clone()));
    println!("Hashmap: {}", similar_pairs_hashmap(words.clone()));
    println!("Hashmap: {}", similar_pairs_hashmap2(words.clone()));
    println!("String Key: {}", similar_pairs_string_key(words));

    let words2 = vec!["aabb".to_string(), "ab".to_string(), "ba".to_string()];
    println!("Bitmask: {}", similar_pairs_bitmask(words2.clone()));
    println!("Hashmap: {}", similar_pairs_hashmap(words2.clone()));
    println!("String Key: {}", similar_pairs_string_key(words2));
}
