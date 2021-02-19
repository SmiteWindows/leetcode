// https://leetcode-cn.com/problems/ransom-note/
// Runtime: 8 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut hm: HashMap<char, i32> = HashMap::new();
    for c in ransom_note.chars() {
        *hm.entry(c).or_default() += 1;
    }
    let len = hm.len();
    let mut count = 0;
    for c in magazine.chars() {
        if let Some(v) = hm.get_mut(&c) {
            *v -= 1;
            if *v == 0 {
                count += 1;
            }
        }
    }
    count == len
}
// string
#[test]
fn test1_383() {
    assert_eq!(can_construct("a".to_string(), "b".to_string()), false);
    assert_eq!(can_construct("aa".to_string(), "ab".to_string()), false);
    assert_eq!(can_construct("aa".to_string(), "aab".to_string()), true);
}
