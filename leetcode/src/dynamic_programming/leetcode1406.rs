// https://leetcode-cn.com/problems/stone-game-iii/
// Runtime: 148 ms
// Memory Usage: 9.2 MB
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashMap,
};
pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
    let n = stone_value.len();
    let mut memo = HashMap::new();
    match dp(0, &mut memo, &stone_value, n).cmp(&0) {
        Equal => "Tie".to_string(),
        Greater => "Alice".to_string(),
        Less => "Bob".to_string(),
    }
}

fn dp(start: usize, memo: &mut HashMap<usize, i32>, values: &[i32], n: usize) -> i32 {
    if let Some(&res) = memo.get(&start) {
        return res;
    }
    let res = if start == n {
        0
    } else {
        let mut sum = 0;
        let mut max = i32::MIN;
        for i in start..(start + 3).min(n) {
            sum += values[i];
            max = max.max(sum - dp(i + 1, memo, values, n));
        }
        max
    };
    memo.insert(start, res);
    res
}
// dynamic_programming
#[test]
fn test1_1406() {
    assert_eq!(stone_game_iii(vec![1, 2, 3, 7]), String::from("Bob"));
    assert_eq!(stone_game_iii(vec![1, 2, 3, -9]), String::from("Alice"));
    assert_eq!(stone_game_iii(vec![1, 2, 3, 6]), String::from("Tie"));
    assert_eq!(
        stone_game_iii(vec![1, 2, 3, -1, -2, -3, 7]),
        String::from("Alice")
    );
    assert_eq!(stone_game_iii(vec![-1, -2, -3]), String::from("Tie"));
}
