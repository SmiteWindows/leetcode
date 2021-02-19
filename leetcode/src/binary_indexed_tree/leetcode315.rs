// https://leetcode-cn.com/problems/count-of-smaller-numbers-after-self/
use std::collections::BTreeMap;
pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
    let mut count: BTreeMap<i32, usize> = BTreeMap::new();
    let n = nums.len();
    let mut res = vec![0; n];
    for i in (0..n).rev() {
        let mut sum = 0;
        let x = nums[i];
        for (_, v) in count.range(..x) {
            sum += v;
        }
        *count.entry(x).or_default() += 1;
        res[i] = sum as i32;
    }
    res
}
// divide_and_conquer binary_indexed_tree segment_tree binary_search sort
#[test]
fn test3_315() {
    assert_eq!(count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
}
