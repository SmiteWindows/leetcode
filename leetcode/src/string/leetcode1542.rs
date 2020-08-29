// https://leetcode.com/problems/find-longest-awesome-substring/
// Runtime: 100 ms
// Memory Usage: 2.3 MB
use std::collections::HashMap;
pub fn longest_awesome(s: String) -> i32 {
    let mut mask = 0;
    let mut hm: HashMap<u32, usize> = HashMap::new();
    hm.insert(0, 0);
    let mut res = 0;
    for (i, b) in s.bytes().enumerate() {
        mask ^= 1 << (b - b'0');
        if let Some(j) = hm.get(&mask) {
            res = res.max(i + 1 - j);
        }
        for k in 0..10 {
            if let Some(j) = hm.get(&(mask ^ (1 << k))) {
                res = res.max(i + 1 - j);
            }
        }
        hm.entry(mask).or_insert(i + 1);
    }
    res as i32
}
// string bit_manipulation
#[test]
fn test1_1542() {
    assert_eq!(longest_awesome("3242415".to_string()), 5);
    assert_eq!(longest_awesome("12345678".to_string()), 1);
    assert_eq!(longest_awesome("213123".to_string()), 6);
    assert_eq!(longest_awesome("00".to_string()), 2);
}
