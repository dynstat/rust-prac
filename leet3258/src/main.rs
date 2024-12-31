// use std::collections::{HashMap, HashSet};

fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
    let s_len = s.len();
    // let mut left = 0;
    let mut count = 0;
    let s: Vec<char>= s.chars().collect();
    
    for start in 0..s_len {
        let mut zeroes = 0;
        let mut ones = 0;
        // expanding the window and counting the substring using the 
        // formula, i.e. count = right - left + 1
        for end in start..s_len {
            if s[end] == '0' {
                zeroes += 1;
            }
            else {
                ones += 1;
            }

            // while zeroes > k && ones > k {
            //     let val = s[left];
            //     if val == '0'{
            //         zeroes -= 1;
            //     }
            //     else {
            //         ones -= 1;
            //     }
            //      left += 1;
            // }
            if zeroes > k && ones > k {
                break;
            }
            else {
                count += 1;
            }
        }
    }
    count
    // todo!();
}

fn main() {
    println!("{}", count_k_constraint_substrings("10101".to_string(), 1));
    println!(
        "{}",
        count_k_constraint_substrings("1010101".to_string(), 2)
    );
}
