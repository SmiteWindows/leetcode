// https://leetcode-cn.com/problems/dice-roll-simulation/
// Runtime: 36 ms
// Memory Usage: 4.3 MB
const MOD: i32 = 1_000_000_007;
pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut dp = vec![vec![vec![0; 5001]; 16]; 6];
    dfs(n, -1, 1, &mut dp, &roll_max)
}

fn dfs(n: usize, prev: i32, repeat: usize, dp: &mut Vec<Vec<Vec<i32>>>, roll_max: &[i32]) -> i32 {
    if n == 0 {
        1
    } else {
        if prev >= 0 && dp[prev as usize][repeat][n] > 0 {
            return dp[prev as usize][repeat][n];
        }
        let mut sum = 0;
        for i in 0..6 {
            if i == prev {
                if repeat < roll_max[i as usize] as usize {
                    sum += dfs(n - 1, i, repeat + 1, dp, roll_max);
                    sum %= MOD;
                }
            } else {
                sum += dfs(n - 1, i, 1, dp, roll_max);
                sum %= MOD;
            }
        }
        if prev >= 0 {
            dp[prev as usize][repeat][n] = sum;
        }
        sum
    }
}
// dynamic_programming
#[test]
fn test1_1223() {
    assert_eq!(die_simulator(2, vec![1, 1, 2, 2, 2, 3]), 34);
    assert_eq!(die_simulator(2, vec![1, 1, 1, 1, 1, 1]), 30);
    assert_eq!(die_simulator(3, vec![1, 1, 1, 2, 2, 3]), 181);
}
