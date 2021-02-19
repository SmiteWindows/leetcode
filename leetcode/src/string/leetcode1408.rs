// https://leetcode-cn.com/problems/string-matching-in-an-array/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn string_matching(words: Vec<String>) -> Vec<String> {
    let mut res = Vec::new();
    for w in &words {
        if words.iter().any(|w2| w2.contains(w) && w != w2) {
            res.push(w.clone());
        }
    }
    res
}
// string
#[test]
fn test1_1408() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        string_matching(vec_string!["mass", "as", "hero", "superhero"]),
        vec!["as", "hero"]
    );
    assert_eq!(
        string_matching(vec_string!["leetcode", "et", "code"]),
        vec!["et", "code"]
    );
    assert_eq!(
        string_matching(vec_string!["blue", "green", "bu"]),
        vec_string![]
    );
}
