// https://leetcode-cn.com/problems/longest-duplicate-substring/
// Runtime: 160 ms
// Memory Usage: 6.8 MB
use std::collections::HashMap;
const MOD: u64 = 1_000_000_007;
const P: u64 = 26;
pub fn longest_dup_substring(s: String) -> String {
    let mut lo = 0;
    let mut hi = s.len();
    let mut res = "".to_string();
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if let Some(s) = exist(mid, &s, n) {
            res = s;
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    res
}

fn exist(size: usize, s: &[char], n: usize) -> Option<String> {
    let mut pos: HashMap<u64, usize> = HashMap::new();
    let mut hash: u64 = 0;
    let mut pn: u64 = 1;
    for &si in s.iter().take(size) {
        hash *= P;
        hash += (si as u8 - b'a') as u64;
        hash %= MOD;
        pn *= P;
        pn %= MOD;
    }
    pos.insert(hash, size);
    for i in size..n {
        hash *= P;
        hash += (s[i] as u8 - b'a') as u64;
        hash += MOD;
        hash -= (pn * (s[i - size] as u8 - b'a') as u64) % MOD;
        hash %= MOD;
        if let Some(end) = pos.insert(hash, i + 1) {
            if s[end - size..end] == s[i + 1 - size..=i] {
                return Some(s[i + 1 - size..=i].iter().copied().collect::<String>());
            }
        }
    }
    None
}
// binary_search hash_table
#[test]
fn test1_1044() {
    assert_eq!(
        longest_dup_substring("banana".to_string()),
        "ana".to_string()
    );
    assert_eq!(longest_dup_substring("abcd".to_string()), "".to_string());
}
