// https://leetcode-cn.com/problems/domino-and-tromino-tiling/
// Runtime: 0 ms
// Memory Usage: 2 MB
const MOD: usize = 1_000_000_007;
pub fn num_tilings(n: i32) -> i32 {
    let n = n as usize;
    if n == 0 {
        return 0;
    }
    let mut memo = vec![0; n + 1];
    dp(n, &mut memo) as i32
}

fn dp(n: usize, memo: &mut Vec<usize>) -> usize {
    match n {
        0 => 1,
        1 => 1,
        2 => 2,
        3 => 5,
        _ => {
            if memo[n] > 0 {
                return memo[n];
            }
            let mut res = 0;
            res += dp(n - 3, memo);
            res %= MOD;
            res += 2 * dp(n - 1, memo);
            res %= MOD;
            memo[n] = res;
            res
        }
    }
}
// dynamic_programming
#[test]
fn test1_790() {
    assert_eq!(num_tilings(3), 5);
}
