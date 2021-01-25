// https://leetcode-cn.com/problems/can-convert-string-in-k-moves/
// Runtime: 28 ms
// Memory Usage: 3.1 MB
use std::collections::HashMap;
pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
    let n = s.len();
    let m = t.len();
    if n != m {
        return false;
    }
    let s: Vec<i32> = s.bytes().map(|b| (b - b'a') as i32).collect();
    let t: Vec<i32> = t.bytes().map(|b| (b - b'a') as i32).collect();
    let mut count: HashMap<i32, i32> = HashMap::new();
    for i in 0..n {
        if s[i] == t[i] {
            continue;
        }
        *count.entry((26 + t[i] - s[i]) % 26).or_default() += 1;
    }
    let mut max = 0;
    for (k, v) in count {
        max = max.max((v - 1) * 26 + k);
    }
    max <= k
}
// string greedy
#[test]
fn test1_1540() {
    assert_eq!(
        can_convert_string("input".to_string(), "ouput".to_string(), 9),
        true
    );
    assert_eq!(
        can_convert_string("abc".to_string(), "bcd".to_string(), 10),
        false
    );
    assert_eq!(
        can_convert_string("aab".to_string(), "bbb".to_string(), 27),
        true
    );
}
