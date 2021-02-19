// https://leetcode-cn.com/problems/dota2-senate/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
use std::collections::VecDeque;
pub fn predict_party_victory(senate: String) -> String {
    let mut rqueue = VecDeque::new();
    let mut dqueue = VecDeque::new();
    let n = senate.len();
    for (i, c) in senate.char_indices() {
        if c == 'R' {
            rqueue.push_back(i);
        } else {
            dqueue.push_back(i);
        }
    }
    while !rqueue.is_empty() && !dqueue.is_empty() {
        let r = rqueue.pop_front().unwrap();
        let d = dqueue.pop_front().unwrap();
        if r < d {
            rqueue.push_back(r + n);
        } else {
            dqueue.push_back(d + n);
        }
    }
    if rqueue.is_empty() {
        "Dire".to_string()
    } else {
        "Radiant".to_string()
    }
}
// greedy
#[test]
fn test1_649() {
    assert_eq!(
        predict_party_victory("RD".to_string()),
        "Radiant".to_string()
    );
    assert_eq!(predict_party_victory("RDD".to_string()), "Dire".to_string());
}
