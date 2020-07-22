// https://leetcode.com/problems/longest-harmonious-subsequence/
// Runtime: 12 ms
// Memory Usage: 2.3 MB
pub fn find_lhs(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut hs: HashMap<i32, i32> = HashMap::new();
    let mut max = 0;
    for &x in &nums {
        *hs.entry(x).or_default() += 1;
    }
    for (x, u) in &hs {
        if let Some(v) = hs.get(&(x - 1)) {
            max = max.max(u + v);
        }
    }
    max
}
// hash_table
#[test]
fn test1_594() {
    assert_eq!(find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
}
