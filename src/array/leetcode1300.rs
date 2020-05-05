// https://leetcode.com/problems/sum-of-mutated-array-closest-to-target/
pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
    todo!()
}
// binary_search array
#[test]
#[ignore]
fn test2_1300() {
    assert_eq!(find_best_value(vec![4, 9, 3], 10), 3);
    assert_eq!(find_best_value(vec![2, 3, 5], 10), 5);
    assert_eq!(
        find_best_value(vec![60864, 25176, 27249, 21296, 20204], 56803),
        11361
    );
}
