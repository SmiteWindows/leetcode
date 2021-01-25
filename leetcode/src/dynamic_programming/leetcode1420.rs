// https://leetcode-cn.com/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons/
// Runtime: 40 ms
// Memory Usage: 7 MB
use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;
pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
    let mut memo = HashMap::new();
    dp(n, m, k, &mut memo) as i32
}

fn dp(n: i32, m: i32, k: i32, memo: &mut HashMap<(i32, i32, i32), i64>) -> i64 {
    if let Some(&res) = memo.get(&(n, m, k)) {
        return res;
    }
    let res = if k == 0 || m < 1 {
        0
    } else if n == 1 {
        if k == 1 {
            m as i64
        } else {
            0
        }
    } else {
        (dp(n, m - 1, k, memo)
            + dp(n - 1, m - 1, k - 1, memo)
            + (dp(n - 1, m, k, memo) + MOD - dp(n - 1, m - 1, k, memo)) * m as i64)
            % MOD
    };
    memo.insert((n, m, k), res);
    res
}
// dynamic_programming
#[test]
fn test1_1420() {
    assert_eq!(num_of_arrays(2, 3, 1), 6);
    assert_eq!(num_of_arrays(5, 2, 3), 0);
    assert_eq!(num_of_arrays(9, 1, 1), 1);
    assert_eq!(num_of_arrays(50, 100, 25), 34549172);
    assert_eq!(num_of_arrays(37, 17, 7), 418930126);
}
