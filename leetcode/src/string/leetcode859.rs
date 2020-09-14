// https://leetcode-cn.com/problems/buddy-strings/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::collections::HashSet;
pub fn buddy_strings(a: String, b: String) -> bool {
    let ac = a.chars().collect::<Vec<_>>();
    let bc = b.chars().collect::<Vec<_>>();
    let a_len = ac.len();
    let b_len = bc.len();
    if a_len != b_len {
        return false;
    }
    if ac == bc {
        let mut hs = HashSet::new();
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
    assert_eq!(buddy_strings("ab".to_string(), "ba".to_string()), true);
    assert_eq!(buddy_strings("ab".to_string(), "ab".to_string()), false);
    assert_eq!(buddy_strings("aa".to_string(), "aa".to_string()), true);
    assert_eq!(
        buddy_strings("aaaaaaabc".to_string(), "aaaaaaacb".to_string()),
        true
    );
    assert_eq!(buddy_strings("".to_string(), "aa".to_string()), false);
}
