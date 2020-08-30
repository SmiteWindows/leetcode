// https://leetcode-cn.com/problems/last-stone-weight-ii/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
    let sum = stones.iter().sum::<i32>() as usize;
    let mut dp = vec![false; sum + 1];
    dp[0] = true;
    let n = stones.len();
    for i in 0..n {
        for j in (1..=sum).rev() {
            if j >= stones[i] as usize && dp[j - stones[i] as usize] {
                dp[j] = true;
            }
        }
    }
    let mut res = sum;
    for (i, &di) in dp.iter().enumerate().take(sum / 2 + 1) {
        if di {
            res = res.min(sum - 2 * i);
        }
    }
    res as i32
}
// dynamic_programming
#[test]
fn test1_1049() {
    assert_eq!(last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]), 1);
}
