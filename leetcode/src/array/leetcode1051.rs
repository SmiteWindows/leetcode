// https://leetcode.com/problems/height-checker/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut sorted = heights.to_vec();
    sorted.sort_unstable();
    heights
        .iter()
        .zip(sorted.iter())
        .fold(0, |sum, (a, b)| if a != b { sum + 1 } else { sum })
}
// array
#[test]
fn test1_1051() {
    assert_eq!(height_checker(vec![1, 1, 4, 2, 1, 3]), 3);
    assert_eq!(height_checker(vec![5, 1, 2, 3, 4]), 5);
    assert_eq!(height_checker(vec![1, 2, 3, 4, 5]), 0);
}
