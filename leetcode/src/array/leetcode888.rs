// https://leetcode-cn.com/problems/fair-candy-swap/
// Runtime: 8 ms
// Memory Usage: 2.4 MB
use std::collections::HashSet;
pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let sum_a: i32 = a.iter().sum();
    let sum_b: i32 = b.iter().sum();
    let hs = b.into_iter().collect::<HashSet<_>>();
    for x in a {
        let y = x + (sum_b - sum_a) / 2;
        if hs.contains(&y) {
            return vec![x, y];
        }
    }
    unreachable!();
}
// array
#[test]
fn test1_888() {
    assert_eq!(fair_candy_swap(vec![1, 1], vec![2, 2]), vec![1, 2]);
    assert_eq!(fair_candy_swap(vec![1, 2], vec![2, 3]), vec![1, 2]);
    assert_eq!(fair_candy_swap(vec![2], vec![1, 3]), vec![2, 3]);
    assert_eq!(fair_candy_swap(vec![1, 2, 5], vec![2, 4]), vec![5, 4]);
}
