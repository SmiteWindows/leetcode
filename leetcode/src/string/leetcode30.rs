// https://leetcode-cn.com/problems/substring-with-concatenation-of-all-words/
// Runtime: 44 ms
// Memory Usage: 2.4 MB
use std::collections::HashMap;
pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let n = s.len();
    if n == 0 {
        return vec![];
    }
    let mut count: HashMap<&str, usize> = HashMap::new();
    let m = words.len();
    if m == 0 {
        return vec![];
    }
    let size = words[0].len();
    if m * size > n {
        return vec![];
    }
    for w in &words {
        *count.entry(w).or_default() += 1;
    }
    let mut res = vec![];
    'outer: for i in 0..=n - m * size {
        let mut cur: HashMap<&str, usize> = HashMap::new();
        for j in 0..m {
            let w = &s[i + j * size..i + (j + 1) * size];
            if let Some(x) = count.get(w) {
                let y = cur.entry(w).or_default();
                *y += 1;
                if *y > *x {
                    continue 'outer;
                }
            } else {
                continue 'outer;
            }
        }
        res.push(i as i32);
    }
    res
}
// string two_pointers hash_table
#[test]
fn test2_30() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        find_substring("barfoothefoobarman".to_string(), vec_string!["foo", "bar"]),
        vec![0, 9]
    );
    assert_eq!(
        find_substring(
            "wordgoodgoodgoodbestword".to_string(),
            vec_string!["word", "good", "best", "word"]
        ),
        vec![]
    );
}
