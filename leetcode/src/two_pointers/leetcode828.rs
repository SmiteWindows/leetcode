// https://leetcode-cn.com/problems/count-unique-characters-of-all-substrings-of-a-given-string/
// Runtime: 8 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
const MOD: i64 = 1_000_000_007;
pub fn unique_letter_string(s: String) -> i32 {
    let n = s.len();
    let s: Vec<char> = s.chars().collect();
    let mut idx: HashMap<char, Vec<usize>> = HashMap::new();
    for i in 0..n {
        idx.entry(s[i]).or_default().push(i + 1);
    }
    let mut res = 0;
    for v in idx.values() {
        let m = v.len();
        for i in 0..m {
            let prev = if i > 0 { v[i - 1] } else { 0 };
            let next = if i + 1 < m { v[i + 1] } else { n + 1 };
            res += ((v[i] - prev) * (next - v[i])) as i64;
            res %= MOD;
        }
    }
    res as i32
}
// two_pointers
#[test]
fn test1_828() {
    assert_eq!(unique_letter_string("ABC".to_string()), 10);
    assert_eq!(unique_letter_string("ABA".to_string()), 8);
    assert_eq!(unique_letter_string("LEETCODE".to_string()), 92);
}
