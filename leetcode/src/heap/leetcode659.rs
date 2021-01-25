// https://leetcode-cn.com/problems/split-array-into-consecutive-subsequences/
// Runtime: 24 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn is_possible(nums: Vec<i32>) -> bool {
    let mut left: HashMap<i32, usize> = HashMap::new();
    let mut end: HashMap<i32, usize> = HashMap::new();
    for &x in &nums {
        *left.entry(x).or_default() += 1;
    }
    for &x in &nums {
        if *left.entry(x).or_default() == 0 {
            continue;
        }
        if *end.entry(x - 1).or_default() > 0 {
            *left.entry(x).or_default() -= 1;
            *end.entry(x - 1).or_default() -= 1;
            *end.entry(x).or_default() += 1;
            continue;
        }
        if *left.entry(x + 1).or_default() > 0 && *left.entry(x + 2).or_default() > 0 {
            *left.entry(x).or_default() -= 1;
            *left.entry(x + 1).or_default() -= 1;
            *left.entry(x + 2).or_default() -= 1;
            *end.entry(x + 2).or_default() += 1;
            continue;
        }
        return false;
    }
    true
}
// heap greedy
#[test]
fn test1_659() {
    assert_eq!(is_possible(vec![1, 2, 3, 3, 4, 5]), true);
    assert_eq!(is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]), true);
    assert_eq!(is_possible(vec![1, 2, 3, 4, 4, 5]), false);
}
