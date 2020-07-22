// https://leetcode.com/problems/array-partition-i/
// Runtime: 12 ms
// Memory Usage: 2.2 MB
pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    nums.chunks(2).fold(0, |sum, pair| sum + pair[0])
}
// array
#[test]
fn test1_561() {
    assert_eq!(array_pair_sum(vec![1, 4, 3, 2]), 4);
}
