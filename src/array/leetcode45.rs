// https://leetcode.com/problems/jump-game-ii/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn jump(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut end = 0;
    let mut max = 0;
    let mut res = 0;
    for (i, &num) in nums.iter().enumerate().take(n - 1) {
        max = max.max(i + num as usize);
        if i == end {
            res += 1;
            end = max;
        }
    }
    res
}
// greedy array
#[test]
fn test2_45() {
    assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
}
