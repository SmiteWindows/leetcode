// https://leetcode.com/problems/minimum-window-substring/
#![allow(clippy::many_single_char_names)]
use std::collections::HashMap;
pub fn min_window(s: String, t: String) -> String {
    if s.is_empty() || t.is_empty() {
        return String::from("");
    }
    let mut dict_t: HashMap<char, i32> = HashMap::new();
    for i in 0..t.len() {
        let c = t.chars().nth(i).unwrap();
        *dict_t.entry(c).or_default() += 1;
    }
    let required = dict_t.len();
    let mut filtered_s = Vec::new();
    for i in 0..s.len() {
        let c = s.chars().nth(i).unwrap();
        if dict_t.contains_key(&c) {
            filtered_s.push((i, c));
        }
    }
    let (mut l, mut r) = (0, 0);
    let mut formed = 0;
    let mut window_counts: HashMap<char, i32> = HashMap::new();
    let mut res = [std::usize::MAX, 0, 0];
    while r < filtered_s.len() {
        let mut c = filtered_s[r].1;
        *window_counts.entry(c).or_default() += 1;
        if dict_t.contains_key(&c) && window_counts[&c] == dict_t[&c] {
            formed += 1;
        }
        while l <= r && formed == required {
            c = filtered_s[l].1;
            let end = filtered_s[r].0;
            let start = filtered_s[l].0;
            if end - start + 1 < res[0] {
                res[0] = end - start + 1;
                res[1] = start;
                res[2] = end;
            }
            *window_counts.entry(c).or_default() -= 1; // TODO
            if dict_t.contains_key(&c) && window_counts[&c] < dict_t[&c] {
                formed -= 1;
            }
            l += 1;
        }
        r += 1;
    }
    if res[0] == std::usize::MAX {
        String::from("")
    } else {
        s[res[1]..=res[2]].to_string()
    }
}
// hash_table two_pointers string sliding_window
#[test]
fn test1_76() {
    assert_eq!(
        min_window(String::from("ADOBECODEBANC"), String::from("ABC")),
        String::from("BANC")
    );
}
