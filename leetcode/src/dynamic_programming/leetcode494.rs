// https://leetcode-cn.com/problems/target-sum/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
    let sum: i32 = nums.iter().sum();
    if sum < s {
        return 0;
    }
    let sum = s + sum;
    if sum & 1 == 1 {
        return 0;
    }
    let sum = sum / 2;
    let mut dp = vec![0; sum as usize + 1];
    dp[0] = 1;
    for num in nums {
        for i in (num as usize..sum as usize + 1).rev() {
            dp[i] += dp[i - num as usize];
        }
    }
    dp[sum as usize]
}
// dynamic_programming depth_first_search
#[test]
fn test1_494() {
    assert_eq!(find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
}
