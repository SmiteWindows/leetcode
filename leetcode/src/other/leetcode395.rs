// https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
pub fn longest_substring(s: String, k: i32) -> i32 {
    let mut hm: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *hm.entry(c).or_default() += 1;
    }
    for (c, v) in hm {
        if v < k as usize {
            return s
                .split_terminator(c)
                .map(|s| longest_substring(s.to_string(), k))
                .max()
                .unwrap();
        }
    }
    s.len() as i32
}
#[test]
fn test395() {
    assert_eq!(longest_substring("aaabb".to_string(), 3), 3);
    assert_eq!(longest_substring("ababbc".to_string(), 2), 5);
    assert_eq!(longest_substring("ababbc".to_string(), 3), 0);
}
