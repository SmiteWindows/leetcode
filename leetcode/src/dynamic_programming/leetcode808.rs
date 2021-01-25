// https://leetcode-cn.com/problems/soup-servings/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
use std::collections::HashMap;
pub fn soup_servings(n: i32) -> f64 {
    if n > 5000 {
        return 1.0;
    }
    let mut memo = HashMap::new();
    dp(n, n, &mut memo)
}

fn dp(a: i32, b: i32, memo: &mut HashMap<(i32, i32), f64>) -> f64 {
    if a == 0 && b == 0 {
        return 0.5;
    }
    if a == 0 && b != 0 {
        return 1.0;
    }
    if a != 0 && b == 0 {
        return 0.0;
    }

    if let Some(&res) = memo.get(&(a, b)) {
        return res;
    }
    let mut res = 0.0;
    res += 0.25 * dp(a - a.min(100), b, memo);
    res += 0.25 * dp(a - a.min(75), b - b.min(25), memo);
    res += 0.25 * dp(a - a.min(50), b - b.min(50), memo);
    res += 0.25 * dp(a - a.min(25), b - b.min(75), memo);
    memo.insert((a, b), res);
    res
}
// dynamic_programming
#[test]
fn test1_808() {
    use leetcode_prelude::assert_approx_eq;
    assert_approx_eq!(soup_servings(50), 0.625);
}
