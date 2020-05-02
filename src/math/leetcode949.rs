// https://leetcode.com/problems/largest-time-for-given-digits/
pub fn largest_time_from_digits(a: Vec<i32>) -> String {
    todo!()
}
// math
#[test]
#[ignore]
fn test1_949() {
    assert_eq!(
        largest_time_from_digits(vec![1, 2, 3, 4]),
        String::from("23:41")
    );
    assert_eq!(largest_time_from_digits(vec![5, 5, 5, 5]), String::from(""));
}
