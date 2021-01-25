// https://leetcode-cn.com/problems/distribute-candies/
// Runtime: 44 ms
// Memory Usage: 2.1 MB
pub fn distribute_candies(candies: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let n = candies.len();
    let mut hs = HashSet::new();
    for val in candies {
        hs.insert(val);
    }
    hs.len().min(n / 2) as i32
}
// hash_table
#[test]
fn test1_575() {
    assert_eq!(distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
    assert_eq!(distribute_candies(vec![1, 1, 2, 3]), 2);
}
