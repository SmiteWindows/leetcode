// https://leetcode-cn.com/problems/construct-k-palindrome-strings/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn can_construct(s: String, k: i32) -> bool {
    let k = k as usize;
    let n = s.len();
    if n < k {
        return false;
    }
    let mut count = vec![0; 26];
    for c in s.bytes() {
        count[(c - b'a') as usize] += 1;
    }
    let mut odd = 0;
    for v in count {
        if v % 2 != 0 {
            odd += 1;
        }
    }
    odd <= k
}
// greedy
#[test]
fn test1_1400() {
    assert_eq!(can_construct("annabelle".to_string(), 2), true);
    assert_eq!(can_construct("leetcode".to_string(), 3), false);
    assert_eq!(can_construct("true".to_string(), 4), true);
    assert_eq!(can_construct("yzyzyzyzyzyzyzy".to_string(), 2), true);
    assert_eq!(can_construct("cr".to_string(), 7), false);
}
