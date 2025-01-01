use std::collections::HashMap;

fn is_valid(left: usize, right: usize, k: i32, curr_map: &HashMap<char, i32>) -> bool {
    let window_size = (right - left + 1) as i32;
    let max_freq = curr_map.values().max().unwrap_or(&0);
    window_size - max_freq <= k
}

pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut l = 0;
    let mut r = 0;
    let s_len = s.len();
    let chars: Vec<char> = s.chars().collect();
    let mut map: HashMap<char, i32> = HashMap::new();
    let mut ans = 0;

    // window expansion
    while r < s_len {
        let right_char = chars[r];
        *map.entry(right_char).or_insert(0) += 1;

        if is_valid(l, r, k, &map) {
            // update the answer
            ans = ans.max((r - l + 1) as i32); // Convert to i32
            r += 1;
        } else {
            let left_char = chars[l];
            if let Some(freq) = map.get_mut(&left_char) {
                *freq -= 1;
                if *freq == 0 {
                    map.remove(&left_char);
                }
            }
            l += 1;
            r += 1; // Also increment r here
        }
    }

    ans
}

fn main() {
    println!("{}", character_replacement("ABAB".to_string(), 2));
    println!("{}", character_replacement("AABABBA".to_string(), 1));
}