// https://leetcode-cn.com/problems/house-robber/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn rob(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .fold((0, 0), |(prev_prev, prev), x| {
            (prev, prev.max(prev_prev + x))
        })
        .1
}
// dynamic_programming
#[test]
fn test1_198() {
    assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
}
