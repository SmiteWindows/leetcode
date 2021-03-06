// https://leetcode-cn.com/problems/number-of-good-pairs/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::collections::HashMap;
pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut hm: HashMap<i32, usize> = HashMap::new();
    let mut res = 0;
    for x in nums {
        let count = hm.entry(x).and_modify(|e| *e += 1).or_default();
        res += *count;
    }
    res as i32
}
// array hash_table math
#[test]
fn test2_1512() {
    assert_eq!(num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
    assert_eq!(num_identical_pairs(vec![1, 1, 1, 1]), 6);
    assert_eq!(num_identical_pairs(vec![1, 2, 3]), 0);
}
