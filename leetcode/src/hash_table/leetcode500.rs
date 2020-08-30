// https://leetcode-cn.com/problems/keyboard-row/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::collections::HashMap;
pub fn find_words(words: Vec<String>) -> Vec<String> {
    let rows = vec![
        String::from("qwertyuiopQWERTYUIOP"),
        String::from("asdfghjklASDFGHJKL"),
        String::from("zxcvbnmZXCVBNM"),
    ];
    let mut hm = HashMap::new();
    for (i, row) in rows.iter().enumerate().take(3) {
        for c in row.chars() {
            hm.insert(c, i);
        }
    }
    let mut res = vec![];
    for word in words {
        let rows = word
            .chars()
            .map(|c| *hm.get(&c).unwrap())
            .collect::<Vec<_>>();
        if rows.windows(2).all(|w| w[0] == w[1]) {
            res.push(word);
        }
    }
    res
}
// hash_table
#[test]
fn test1_500() {
    assert_eq!(
        find_words(vec![
            String::from("Hello"),
            String::from("Alaska"),
            String::from("Dad"),
            String::from("Peace")
        ]),
        vec![String::from("Alaska"), String::from("Dad")]
    );
}
