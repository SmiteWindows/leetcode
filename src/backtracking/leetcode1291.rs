// https://leetcode.com/problems/sequential-digits/
pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    todo!()
}
// backtracking
#[test]
#[ignore]
fn test1_1240() {
    assert_eq!(sequential_digits(100, 300), vec![123, 234]);
    assert_eq!(
        sequential_digits(1000, 13000),
        vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]
    );
}
