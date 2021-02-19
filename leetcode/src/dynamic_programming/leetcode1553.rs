// https://leetcode-cn.com/problems/minimum-number-of-days-to-eat-n-oranges/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn min_days(n: i32) -> i32 {
    let mut memo: HashMap<i32, i32> = HashMap::new();
    dp(n, &mut memo)
}

fn dp(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if n <= 1 {
        n
    } else {
        if let Some(&res) = memo.get(&n) {
            return res;
        }
        let res = 1 + (n % 2 + dp(n / 2, memo)).min(n % 3 + dp(n / 3, memo));
        memo.insert(n, res);
        res
    }
}
// dynamic_programming
#[test]
fn test1_1553() {
    assert_eq!(min_days(10), 4);
    assert_eq!(min_days(6), 3);
    assert_eq!(min_days(1), 1);
    assert_eq!(min_days(56), 6);
}
