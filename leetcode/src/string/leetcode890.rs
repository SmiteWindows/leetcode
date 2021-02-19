// https://leetcode-cn.com/problems/find-and-replace-pattern/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
    let n = pattern.len();
    let pattern = pattern.chars().collect::<Vec<_>>();
    let mut res = Vec::new();
    for word in words {
        let word = word.chars().collect::<Vec<_>>();
        if matches(&word, &pattern, n) {
            res.push(word.iter().collect());
        }
    }
    res
}

fn matches(word: &[char], pattern: &[char], n: usize) -> bool {
    if word.len() != n {
        return false;
    }
    let mut hm1 = HashMap::new();
    let mut hm2 = HashMap::new();
    for i in 0..n {
        let c1 = word[i];
        let c2 = pattern[i];
        if let Some(old) = hm1.insert(c1, c2) {
            if old != c2 {
                return false;
            }
        }
        if let Some(old) = hm2.insert(c2, c1) {
            if old != c1 {
                return false;
            }
        }
    }
    true
}
// string
#[test]
fn test1_890() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        find_and_replace_pattern(
            vec_string!["abc", "deq", "mee", "aqq", "dkd", "ccc"],
            "abb".to_string()
        ),
        vec_string!["mee", "aqq"]
    );
}
