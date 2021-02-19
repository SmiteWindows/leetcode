// https://leetcode-cn.com/problems/jump-game/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
pub fn can_jump(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut last = n - 1;
    for i in (0..n).rev() {
        if i + nums[i] as usize >= last {
            last = i;
        }
    }
    last == 0
}
// greedy array
#[test]
fn test2_55() {
    assert_eq!(can_jump(vec![2, 3, 1, 1, 4]), true);
    assert_eq!(can_jump(vec![3, 2, 1, 0, 4]), false);
}
