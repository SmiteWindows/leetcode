// https://leetcode-cn.com/problems/integer-break/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
pub fn integer_break(n: i32) -> i32 {
    let mut memo = HashMap::new();
    dp(n, true, &mut memo)
}

fn dp(n: i32, split: bool, memo: &mut HashMap<(i32, bool), i32>) -> i32 {
    if let Some(&res) = memo.get(&(n, split)) {
        return res;
    }
    let mut res = if split { 0 } else { n };
    for i in 1..n {
        res = res.max(i * dp(n - i, false, memo));
    }
    memo.insert((n, split), res);
    res
}
// math dynamic_programming
#[test]
fn test1_343() {
    assert_eq!(integer_break(2), 1);
    assert_eq!(integer_break(10), 36);
}
