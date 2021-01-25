// https://leetcode-cn.com/problems/reconstruct-original-digits-from-english/
// Runtime: 8 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn original_digits(s: String) -> String {
    let digits = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits = digits
        .into_iter()
        .map(|s| {
            let mut hm: HashMap<char, usize> = HashMap::new();
            for c in s.chars() {
                *hm.entry(c).or_default() += 1;
            }
            hm
        })
        .collect::<Vec<_>>();
    let mut count: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *count.entry(c).or_default() += 1;
    }
    let mut res = vec![];
    for i in vec![0, 4, 5, 6, 7, 8, 3, 2, 1, 9].into_iter() {
        let mut min = usize::MAX;
        for (&c, &v) in &digits[i] {
            if *count.entry(c).or_default() % v != 0 {
                continue;
            }
            if *count.entry(c).or_default() / v < min {
                min = *count.entry(c).or_default() / v;
            }
        }
        if min != usize::MAX {
            for (&c, &v) in &digits[i] {
                *count.entry(c).or_default() -= min * v;
            }
            res.resize(min + res.len(), (b'0' + i as u8) as char);
        }
    }
    res.sort_unstable();
    res.into_iter().collect()
}
// math
#[test]
fn test1_423() {
    assert_eq!(original_digits("owoztneoer".to_string()), "012".to_string());
    assert_eq!(original_digits("fviefuro".to_string()), "45".to_string());
}
