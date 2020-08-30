// https://leetcode-cn.com/problems/string-matching-in-an-array/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn string_matching(words: Vec<String>) -> Vec<String> {
    let mut res = vec![];
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
    assert_eq!(
        string_matching(vec![
            String::from("mass"),
            String::from("as"),
            String::from("hero"),
            String::from("superhero")
        ]),
        vec![String::from("as"), String::from("hero")]
    );
    assert_eq!(
        string_matching(vec![
            String::from("leetcode"),
            String::from("et"),
            String::from("code")
        ]),
        vec![String::from("et"), String::from("code")]
    );
    assert_eq!(
        string_matching(vec![
            String::from("blue"),
            String::from("green"),
            String::from("bu")
        ]),
        vec![] as Vec<String>
    );
}
