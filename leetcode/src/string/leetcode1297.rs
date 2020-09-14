// https://leetcode-cn.com/problems/maximum-number-of-occurrences-of-a-substring/
// Runtime: 8 ms
// Memory Usage: 4 MB
use std::collections::HashMap;
pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
    let min_size = min_size as usize;
    let max_letters = max_letters as usize;
    let n = s.len();
    let s = s.bytes().map(|b| (b - b'a') as usize).collect::<Vec<_>>();
    let mut hm: HashMap<u32, usize> = HashMap::new();
    let mut count = vec![0; 26];
    let mut letters = 0;
    let mut hash = 0;
    let pow = 26_u32.pow(min_size as u32);
    let mut res = 0;
    for i in (0..n).rev() {
        let first = s[i];
        if count[first] == 0 {
            letters += 1;
        }
        count[first] += 1;
        hash *= 26_u32;
        hash += first as u32;
        if i + min_size < n {
            let last = s[i + min_size];
            if count[last] == 1 {
                letters -= 1;
            }
            count[last] -= 1;
            hash -= last as u32 * pow;
        }
        if i + min_size <= n && letters <= max_letters {
            *hm.entry(hash).or_default() += 1;
            res = res.max(hm[&hash]);
        }
    }
    res as i32
}
// bit_manipulation string
#[test]
fn test2_1297() {
    assert_eq!(max_freq("aababcaab"), 2, 3, 4), 2);
    assert_eq!(max_freq("aaaa"), 1, 3, 3), 2);
    assert_eq!(max_freq("aabcabcab"), 2, 2, 3), 3);
    assert_eq!(max_freq("abcde"), 2, 3, 3), 0);
}
