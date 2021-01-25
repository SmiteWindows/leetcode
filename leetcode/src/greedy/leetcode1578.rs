// https://leetcode-cn.com/problems/minimum-deletion-cost-to-avoid-repeating-letters/
// Runtime: 32 ms
// Memory Usage: 2.9 MB
pub fn min_cost(s: String, cost: Vec<i32>) -> i32 {
    let n = s.len();
    let s: Vec<u8> = s.bytes().collect();
    let sum: i32 = cost.iter().sum();
    let mut dp = vec![0; 26];
    let mut max = 0;
    for i in 0..n {
        let b = (s[i] - b'a') as usize;
        dp[b] = dp[b].max((0..26).filter(|&j| j != b).map(|j| dp[j]).max().unwrap() + cost[i]);
        max = max.max(dp[b]);
    }
    sum - max
}
// greedy
#[test]
fn test1_1578() {
    assert_eq!(min_cost("abaac".to_string(), vec![1, 2, 3, 4, 5]), 3);
    assert_eq!(min_cost("abc".to_string(), vec![1, 2, 3]), 0);
    assert_eq!(min_cost("aabaa".to_string(), vec![1, 2, 3, 4, 1]), 2);
}
