// https://leetcode.com/problems/number-of-dice-rolls-with-target-sum/
// Runtime: 44 ms
// Memory Usage: 2.4 MB
use std::collections::HashMap;
pub fn num_rolls_to_target(d: i32, f: i32, target: i32) -> i32 {
    // TODO
    let mut memo = HashMap::new();
    dp(d, f, target, &mut memo)
}

fn dp(d: i32, f: i32, target: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
    if let Some(&val) = memo.get(&(d, target)) {
        return val;
    }
    let res = if d == 0 {
        if target == 0 {
            1
        } else {
            0
        }
    } else {
        let mut sum = 0;
        for i in 1..=f {
            sum += dp(d - 1, f, target - i, memo);
            sum %= 1_000_000_007;
        }
        sum
    };
    memo.insert((d, target), res);
    res
}
// dynamic_programming
#[test]
fn test1_1155() {
    assert_eq!(num_rolls_to_target(1, 6, 3), 1);
    assert_eq!(num_rolls_to_target(2, 6, 7), 6);
    assert_eq!(num_rolls_to_target(2, 5, 10), 1);
    assert_eq!(num_rolls_to_target(1, 2, 3), 0);
    assert_eq!(num_rolls_to_target(30, 30, 500), 222616187);
}
