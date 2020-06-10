// https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts/
// Runtime: 64 ms
// Memory Usage: 3.1 MB
use std::collections::HashMap;
pub fn find_the_longest_substring(s: String) -> i32 {
    let mut mask = 0;
    let mut hm = HashMap::new();
    hm.insert(0, 0);
    let mut res = 0;
    for (i, c) in s.char_indices() {
        if vowel(c) != 0 {
            mask ^= 1 << vowel(c);
        }
        hm.entry(mask).or_insert(i + 1);
        res = res.max(i + 1 - hm[&mask]);
    }
    res as i32
}

fn vowel(c: char) -> usize {
    match c {
        'a' => 1,
        'e' => 2,
        'i' => 3,
        'o' => 4,
        'u' => 5,
        _ => 0,
    }
}
// string
#[test]
fn test1_1371() {
    assert_eq!(
        find_the_longest_substring(String::from("eleetminicoworoep")),
        13
    );
    assert_eq!(
        find_the_longest_substring(String::from("leetcodeisgreat")),
        5
    );
    assert_eq!(find_the_longest_substring(String::from("bcbcbc")), 6);
}
