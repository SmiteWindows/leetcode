// https://leetcode.com/problems/uncommon-words-from-two-sentences/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::BTreeMap;
pub fn uncommon_from_sentences(a: String, b: String) -> Vec<String> {
    let mut hs: BTreeMap<&str, i32> = BTreeMap::new();
    a.split_whitespace().for_each(|s| {
        *hs.entry(s).or_default() += 1;
    });
    b.split_whitespace().for_each(|s| {
        *hs.entry(s).or_default() += 1;
    });
    let mut res = vec![];
    for (s, v) in hs {
        if v == 1 {
            res.push(s.to_string());
        }
    }
    res
}
// hash_table
#[test]
fn test1_884() {
    assert_eq!(
        uncommon_from_sentences(
            String::from("this apple is sweet"),
            String::from("this apple is sour")
        ),
        // vec![String::from("sweet"), String::from("sour")]
        vec![String::from("sour"), String::from("sweet")]
    );
    assert_eq!(
        uncommon_from_sentences(String::from("apple apple"), String::from("banana")),
        vec![String::from("banana")]
    );
}
