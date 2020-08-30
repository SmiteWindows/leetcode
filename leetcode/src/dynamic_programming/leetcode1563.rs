// https://leetcode-cn.com/problems/stone-game-v/
// Runtime: 192 ms
// Memory Usage: 5.1 MB
use std::collections::HashMap;
pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
    let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
    let n = stone_value.len();
    dp(0, n, &mut memo, &stone_value)
}

fn dp(start: usize, end: usize, memo: &mut HashMap<(usize, usize), i32>, stones: &[i32]) -> i32 {
    if end - start < 2 {
        return 0;
    }
    if let Some(&res) = memo.get(&(start, end)) {
        return res;
    }
    let mut left = 0;
    let mut right = stones[start..end].iter().sum::<i32>();
    let mut res = 0;
    for i in start..end {
        left += stones[i];
        right -= stones[i];
        if left > right {
            res = res.max(right + dp(i + 1, end, memo, stones));
            continue;
        }
        if left < right {
            res = res.max(left + dp(start, i + 1, memo, stones));
            continue;
        }
        res = res.max(left + dp(start, i + 1, memo, stones));
        res = res.max(right + dp(i + 1, end, memo, stones));
    }
    memo.insert((start, end), res);
    res
}
// dynamic_programming
#[test]
fn test1_1563() {
    assert_eq!(stone_game_v(vec![6, 2, 3, 4, 5, 5]), 18);
    assert_eq!(stone_game_v(vec![7, 7, 7, 7, 7, 7, 7]), 28);
    assert_eq!(stone_game_v(vec![4]), 0);
}
