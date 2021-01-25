// https://leetcode-cn.com/problems/maximum-points-you-can-obtain-from-cards/
// Runtime: 4 ms
// Memory Usage: 3.2 MB
pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let n = card_points.len();
    let mut sum = card_points.iter().take(k).sum::<i32>();
    let mut max = sum;
    for i in 0..k {
        sum -= card_points[k - 1 - i];
        sum += card_points[n - 1 - i];
        max = max.max(sum);
    }
    max
}
// array dynamic_programming sliding_window
#[test]
fn test3_1423() {
    assert_eq!(max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
    assert_eq!(max_score(vec![2, 2, 2], 2), 4);
    assert_eq!(max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
    assert_eq!(max_score(vec![1, 1000, 1], 1), 1);
    assert_eq!(max_score(vec![1, 79, 80, 1, 1, 1, 200, 1], 3), 202);
}
