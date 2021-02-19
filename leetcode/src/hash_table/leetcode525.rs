// https://leetcode-cn.com/problems/contiguous-array/
// Runtime: 32 ms
// Memory Usage: 3.4 MB
use std::collections::HashMap;
pub fn find_max_length(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut hm: HashMap<i32, usize> = HashMap::new();
    let mut diff = 0;
    hm.entry(0).or_default();
    for (i, &num) in nums.iter().enumerate() {
        if num == 1 {
            diff += 1;
        } else {
            diff -= 1;
        }
        let j = *hm.entry(diff).or_insert(i + 1);
        res = res.max(i + 1 - j);
    }
    res as i32
}
// hash_table
#[test]
fn test1_525() {
    assert_eq!(find_max_length(vec![0, 1]), 2);
    assert_eq!(find_max_length(vec![0, 1, 0]), 2);
}
