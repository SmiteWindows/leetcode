// https://leetcode-cn.com/problems/allocate-mailboxes/
// Runtime: 20 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn min_distance(houses: Vec<i32>, k: i32) -> i32 {
    let mut houses = houses;
    let n = houses.len();
    let k = k as usize;
    houses.sort_unstable();
    let mut memo = HashMap::new();
    dp(0, k, &mut memo, &houses, n)
}

fn dp(
    start: usize,
    k: usize,
    memo: &mut HashMap<(usize, usize), i32>,
    houses: &[i32],
    n: usize,
) -> i32 {
    if let Some(&res) = memo.get(&(start, k)) {
        return res;
    }
    let res = if k == 1 {
        distance(start, n, houses)
    } else {
        let mut min = i32::MAX;
        for i in start..=n - k {
            let end = i + 1;
            let dist = distance(start, end, houses);
            min = min.min(dist + dp(end, k - 1, memo, houses, n));
        }
        min
    };
    memo.insert((start, k), res);
    res
}

fn distance(start: usize, end: usize, houses: &[i32]) -> i32 {
    let mut sum = 0;
    let median = (houses[(start + end - 1) / 2] + houses[(start + end) / 2]) / 2;
    for house in houses.iter().take(end).skip(start) {
        sum += (house - median).abs();
    }
    sum
}
// math dynamic_programming
#[test]
fn test2_1478() {
    assert_eq!(min_distance(vec![1, 4, 8, 10, 20], 3), 5);
    assert_eq!(min_distance(vec![2, 3, 5, 12, 18], 2), 9);
    assert_eq!(min_distance(vec![7, 4, 6, 1], 1), 8);
    assert_eq!(min_distance(vec![3, 6, 14, 10], 4), 0);
}
