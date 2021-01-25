// https://leetcode-cn.com/problems/out-of-boundary-paths/
// Runtime: 12 ms
// Memory Usage: 3.9 MB
use std::collections::HashMap;
const MOD: i32 = 1_000_000_007;
pub fn find_paths(m: i32, n: i32, k: i32, i: i32, j: i32) -> i32 {
    let mut memo = HashMap::new();
    let m = m as usize;
    let n = n as usize;
    let k = k as usize;
    let i = i as usize;
    let j = j as usize;
    dp(i, j, k, &mut memo, m, n)
}

fn dp(
    i: usize,
    j: usize,
    k: usize,
    memo: &mut HashMap<(usize, usize, usize), i32>,
    n: usize,
    m: usize,
) -> i32 {
    if k == 0 {
        0
    } else {
        if let Some(&res) = memo.get(&(i, j, k)) {
            return res;
        }
        let top = if i > 0 {
            dp(i - 1, j, k - 1, memo, n, m)
        } else {
            1
        };
        let left = if j > 0 {
            dp(i, j - 1, k - 1, memo, n, m)
        } else {
            1
        };
        let bottom = if i + 1 < n {
            dp(i + 1, j, k - 1, memo, n, m)
        } else {
            1
        };
        let right = if j + 1 < m {
            dp(i, j + 1, k - 1, memo, n, m)
        } else {
            1
        };
        let mut res = 0;
        res += top;
        res %= MOD;
        res += left;
        res %= MOD;
        res += right;
        res %= MOD;
        res += bottom;
        res %= MOD;
        memo.insert((i, j, k), res);
        res
    }
}
// dynamic_programming depth_first_search
#[test]
fn test2_576() {
    assert_eq!(find_paths(2, 2, 2, 0, 0), 6);
    assert_eq!(find_paths(1, 3, 3, 0, 1), 12);
}
