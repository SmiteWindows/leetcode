// https://leetcode-cn.com/problems/partition-labels/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::collections::HashMap;
pub fn partition_labels(s: String) -> Vec<i32> {
    let mut res = Vec::new();
    let mut last = HashMap::new();
    for (i, c) in s.char_indices() {
        last.insert(c, i);
    }
    let mut start = 0;
    let mut end = 0;
    for (i, c) in s.char_indices() {
        let j = last[&c];
        if j > end {
            end = j;
        }
        if i == end {
            res.push((end - start + 1) as i32);
            start = last[&c] + 1;
            end = last[&c] + 1;
        }
    }
    res
}
// greedy two_pointers
#[test]
fn test1_763() {
    assert_eq!(
        partition_labels("ababcbacadefegdehijhklij".to_string()),
        vec![9, 7, 8]
    );
}
