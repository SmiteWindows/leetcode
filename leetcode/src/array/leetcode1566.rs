// https://leetcode-cn.com/problems/detect-pattern-of-length-m-repeated-k-or-more-times/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
    let m = m as usize;
    let k = k as usize;
    arr.windows(m * k)
        .any(|w| w.chunks(m).all(|v| *v == w[0..m]))
}
// array
#[test]
fn test1_1566() {
    assert_eq!(contains_pattern(vec![1, 2, 4, 4, 4, 4], 1, 3), true);
    assert_eq!(contains_pattern(vec![1, 2, 1, 2, 1, 1, 1, 3], 2, 2), true);
    assert_eq!(contains_pattern(vec![1, 2, 1, 2, 1, 3], 2, 3), false);
    assert_eq!(contains_pattern(vec![1, 2, 3, 1, 2], 2, 2), false);
    assert_eq!(contains_pattern(vec![2, 2, 2, 2], 2, 3), false);
}
