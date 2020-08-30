// https://leetcode-cn.com/problems/maximum-average-subarray-i/
// Runtime: 16 ms
// Memory Usage: 2.1 MB
pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let n = nums.len();
    let k = k as usize;
    let mut sum = 0;
    for num in nums.iter().take(k) {
        sum += num;
    }
    let mut max = sum;
    for i in k..n {
        sum += nums[i] - nums[i - k];
        max = max.max(sum);
    }
    max as f64 / k as f64
}
// array
#[test]
fn test1_643() {
    assert_eq!(find_max_average(vec![1, 12, -5, -6, 50, 3], 4), 12.75);
}
