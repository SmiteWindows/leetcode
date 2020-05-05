// https://leetcode.com/problems/split-array-into-fibonacci-sequence/
pub fn split_into_fibonacci(s: String) -> Vec<i32> {
    todo!()
}
// backtracking string greedy
#[test]
#[ignore]
fn test2_842() {
    assert_eq!(
        split_into_fibonacci(String::from("123456579")),
        vec![123, 456, 579]
    );
    assert_eq!(
        split_into_fibonacci(String::from("11235813")),
        vec![1, 1, 2, 3, 5, 8, 13]
    );
    assert_eq!(
        split_into_fibonacci(String::from("112358130")),
        vec![String::new()]
    );
    assert_eq!(
        split_into_fibonacci(String::from("0123")),
        vec![String::new()]
    );
    assert_eq!(
        split_into_fibonacci(String::from("1101111")),
        vec![110, 1, 111]
    );
}