// https://leetcode.com/problems/buddy-strings/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::collections::HashSet;
pub fn buddy_strings(a: String, b: String) -> bool {
    let ac: Vec<char> = a.chars().collect();
    let bc: Vec<char> = b.chars().collect();
    let a_len = ac.len();
    let b_len = bc.len();
    if a_len != b_len {
        return false;
    }
    if ac == bc {
        let mut hs: HashSet<char> = HashSet::new();
        let mut sum = 0;
        for &c in &ac {
            if !hs.insert(c) {
                sum += 1;
            }
        }
        sum != 0
    } else {
        let mut pair = vec![];
        for i in 0..a_len {
            if ac[i] != bc[i] {
                pair.push(i);
            }
        }
        if pair.len() == 2 {
            let i = pair[0];
            let j = pair[1];
            ac[i] == bc[j] && ac[j] == bc[i]
        } else {
            false
        }
    }
}
// string
#[test]
fn test1_859() {
    assert_eq!(buddy_strings(String::from("ab"), String::from("ba")), true);
    assert_eq!(buddy_strings(String::from("ab"), String::from("ab")), false);
    assert_eq!(buddy_strings(String::from("aa"), String::from("aa")), true);
    assert_eq!(
        buddy_strings(String::from("aaaaaaabc"), String::from("aaaaaaacb")),
        true
    );
    assert_eq!(buddy_strings(String::from(""), String::from("aa")), false);
}
