// https://leetcode-cn.com/problems/increasing-triplet-subsequence/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut dp = Vec::new();
    for x in nums {
        if let Err(i) = dp.binary_search(&x) {
            if i == dp.len() {
                dp.push(x)
            } else {
                dp[i] = x;
            }
        }
        if dp.len() == 3 {
            return true;
        }
    }
    false
}
#[test]
fn test334() {
    assert_eq!(increasing_triplet(vec![1, 2, 3, 4, 5]), true);
    assert_eq!(increasing_triplet(vec![5, 4, 3, 2, 1]), false);
}
