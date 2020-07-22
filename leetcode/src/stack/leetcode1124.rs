// https://leetcode.com/problems/longest-well-performing-interval/
// Runtime: 4 ms
// Memory Usage: 2.4 MB
use std::collections::HashMap;
pub fn longest_wpi(hours: Vec<i32>) -> i32 {
    let mut hm = HashMap::new();
    let mut score = 0;
    let mut res = 0;
    for (i, &hour) in hours.iter().enumerate() {
        score += if hour > 8 { 1 } else { -1 };
        if score > 0 {
            res = i + 1;
        } else {
            hm.entry(score).or_insert(i);
            if let Some(j) = hm.get(&(score - 1)) {
                res = res.max(i - j);
            }
        }
    }
    res as i32
}
// stack
#[test]
fn test1_1124() {
    assert_eq!(longest_wpi(vec![9, 9, 6, 0, 6, 6, 9]), 3);
}
