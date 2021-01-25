// https://leetcode-cn.com/problems/guess-number-higher-or-lower-ii/
// Runtime: 80 ms
// Memory Usage: 2.7 MB
use std::collections::HashMap;
pub fn get_money_amount(n: i32) -> i32 {
    let mut memo = HashMap::new();
    dp(1, n, &mut memo)
}

fn dp(left: i32, right: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
    if left == right {
        0
    } else {
        if let Some(&res) = memo.get(&(left, right)) {
            return res;
        }
        let mut res = i32::MAX;
        for i in left..right {
            let a = if i != left { dp(left, i - 1, memo) } else { 0 };
            let b = if i != right {
                dp(i + 1, right, memo)
            } else {
                0
            };
            res = res.min(i + a.max(b));
        }
        memo.insert((left, right), res);
        res
    }
}
// dynamic_programming minimax
#[test]
fn test2_375() {
    // assert_eq!(get_money_amount(10), 21);
    assert_eq!(get_money_amount(10), 16);
}
