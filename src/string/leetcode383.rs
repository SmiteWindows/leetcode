// https://leetcode.com/problems/ransom-note/
// Runtime: 8 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut hm: HashMap<char, i32> = HashMap::new();
    if ransom_note.len() > magazine.len() {
        return false;
    }
    if ransom_note == magazine {
        return true;
    }
    for c in magazine.chars() {
        *hm.entry(c).or_default() += 1;
    }
    for c in ransom_note.chars() {
        if let Some(v) = hm.get_mut(&c) {
            *v -= 1;
            if *v < 0 {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}
// string
#[test]
fn test1_383() {
    assert_eq!(can_construct(String::from("a"), String::from("b")), false);
    assert_eq!(can_construct(String::from("aa"), String::from("ab")), false);
    assert_eq!(can_construct(String::from("aa"), String::from("aab")), true);
}
