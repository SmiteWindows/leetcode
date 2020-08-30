// https://leetcode-cn.com/problems/minimum-cost-to-cut-a-stick/
// Runtime: 156 ms
// Memory Usage: 2.3 MB
use std::collections::HashMap;
pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
    let mut cuts = cuts;
    cuts.sort_unstable();
    let mut memo: HashMap<(i32, i32), i32> = HashMap::new();
    dp(0, n, &mut memo, &cuts)
}

fn dp(left: i32, right: i32, memo: &mut HashMap<(i32, i32), i32>, cuts: &[i32]) -> i32 {
    if let Some(&res) = memo.get(&(left, right)) {
        return res;
    }
    let mut cuts_inside = vec![];
    for &x in cuts {
        if x > left && x < right {
            cuts_inside.push(x);
        }
    }
    let res = if !cuts_inside.is_empty() {
        let mut min = i32::MAX;
        for x in cuts_inside {
            min = min.min(dp(left, x, memo, cuts) + dp(x, right, memo, cuts));
        }
        min + right - left
    } else {
        0
    };
    memo.insert((left, right), res);
    res
}
// dynamic_programming
#[test]
fn test1_1547() {
    assert_eq!(min_cost(7, vec![1, 3, 4, 5]), 16);
    assert_eq!(min_cost(9, vec![5, 6, 1, 4, 2]), 22);
}
