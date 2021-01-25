// https://leetcode-cn.com/problems/largest-multiple-of-three/
pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
    todo!()
}
// math dynamic_programming
#[test]
#[ignore]
fn test1_1363() {
    assert_eq!(largest_multiple_of_three(vec![8, 1, 9]), "981".to_string());
    assert_eq!(
        largest_multiple_of_three(vec![8, 6, 7, 1, 0]),
        "8760".to_string()
    );
    assert_eq!(largest_multiple_of_three(vec![1]), "".to_string());
    assert_eq!(
        largest_multiple_of_three(vec![0, 0, 0, 0, 0, 0]),
        "0".to_string()
    );
}
