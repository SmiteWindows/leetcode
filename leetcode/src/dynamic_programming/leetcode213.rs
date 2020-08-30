// https://leetcode-cn.com/problems/house-robber-ii/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return nums[0];
    }
    rob_slice(&nums[0..n - 1]).max(rob_slice(&nums[1..n]))
}

fn rob_slice(v: &[i32]) -> i32 {
    let n = v.len();
    let mut prev = 0;
    let mut curr = 0;
    for vi in v.iter().take(n) {
        let temp = curr.max(vi + prev);
        prev = curr;
        curr = temp;
    }
    curr
}
// dynamic_programming
#[test]
fn test1_213() {
    assert_eq!(rob(vec![2, 3, 2]), 3);
    assert_eq!(rob(vec![1, 2, 3, 1]), 4);
}
