// https://leetcode.com/problems/largest-multiple-of-three/
pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
    todo!()
}
// math dynamic_programming
#[test]
#[ignore]
fn test2_1363() {
    assert_eq!(
        largest_multiple_of_three(vec![8, 1, 9]),
        String::from("981")
    );
    assert_eq!(
        largest_multiple_of_three(vec![8, 6, 7, 1, 0]),
        String::from("8760")
    );
    assert_eq!(largest_multiple_of_three(vec![1]), String::from(""));
    assert_eq!(
        largest_multiple_of_three(vec![0, 0, 0, 0, 0, 0]),
        String::from("0")
    );
}
