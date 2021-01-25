// https://leetcode-cn.com/problems/partition-equal-subset-sum/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn can_partition(nums: Vec<i32>) -> bool {
    let sum = nums.iter().sum::<i32>();
    if sum % 2 == 1 {
        return false;
    }
    let half = (sum / 2) as usize;
    let mut dp = vec![false; half as usize + 1];
    dp[0] = true;
    let mut max = 0;
    for x in nums {
        let j = x as usize;
        for i in (j..=half.min(max + j)).rev() {
            if dp[i - j] {
                dp[i] = true;
                max = max.max(i);
            }
        }
        if dp[half] {
            return true;
        }
    }
    false
}
// dynamic_programming
#[test]
fn test1_416() {
    assert_eq!(can_partition(vec![1, 5, 11, 5]), true);
    assert_eq!(can_partition(vec![1, 2, 3, 5]), false);
}
