// https://leetcode-cn.com/problems/uncommon-words-from-two-sentences/
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
    let mut res = Vec::new();
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
    use leetcode_prelude::vec_string;
    assert_eq!(
        uncommon_from_sentences(
            "this apple is sweet".to_string(),
            "this apple is sour".to_string()
        ),
        // vec!["sweet", "sour")]
        vec_string!["sour", "sweet"]
    );
    assert_eq!(
        uncommon_from_sentences("apple apple".to_string(), "banana".to_string()),
        vec_string!["banana"]
    );
}
