// https://leetcode.com/problems/tiling-a-rectangle-with-the-fewest-squares/
// Runtime: 0 ms
// Memory Usage: 2 MB
#![allow(clippy::many_single_char_names)]
use std::collections::HashMap;
pub fn tiling_rectangle(n: i32, m: i32) -> i32 {
    let mut memo: HashMap<(i32, i32), i32> = HashMap::new();
    dp(n, m, &mut memo)
}
fn dp(n: i32, m: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
    if n == m {
        1
    } else {
        if let Some(&res) = memo.get(&(n, m)) {
            return res;
        }
        let mut res = i32::MAX;

        for i in 1..n {
            res = res.min(dp(i, m, memo) + dp(n - i, m, memo));
        }

        for j in 1..m {
            res = res.min(dp(n, j, memo) + dp(n, m - j, memo));
        }

        for i in 1..n - 1 {
            for j in 1..m - 1 {
                let a = dp(i, j + 1, memo);
                let b = dp(i + 1, m - j - 1, memo);
                let c = dp(n - i, j, memo);
                let d = dp(n - i - 1, m - j, memo);
                res = res.min(a + b + c + d + 1);
            }
        }
        memo.insert((n, m), res);
        res
    }
}
// backtracking dynamic_programming
#[test]
fn test2_1240() {
    assert_eq!(tiling_rectangle(2, 3), 3);
    assert_eq!(tiling_rectangle(5, 8), 5);
    assert_eq!(tiling_rectangle(11, 13), 6);
}
