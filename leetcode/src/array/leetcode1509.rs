// https://leetcode-cn.com/problems/minimum-difference-between-largest-and-smallest-value-in-three-moves/?
// Runtime: 36 ms
// Memory Usage: 3.3 MB
pub fn min_difference(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let n = nums.len();
    if n <= 3 {
        return 0;
    }
    nums.sort_unstable();
    let mut res = i32::MAX;
    for i in 0..=3 {
        let min = nums[i..n - (3 - i)].iter().min().unwrap();
        let max = nums[i..n - (3 - i)].iter().max().unwrap();
        res = res.min(max - min);
    }
    res
}
// array sort
#[test]
fn test1_1509() {
    assert_eq!(min_difference(vec![5, 3, 2, 4]), 0);
    assert_eq!(min_difference(vec![1, 5, 0, 10, 14]), 1);
    assert_eq!(min_difference(vec![6, 6, 0, 1, 1, 4, 6]), 2);
    assert_eq!(min_difference(vec![1, 5, 6, 14, 15]), 1);
}
