// https://leetcode.com/problems/find-a-value-of-a-mysterious-function-closest-to-target/
pub fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
    todo!()
}
// binary_search bit_manipulation segment_tree
#[test]
#[ignore]
fn test1_1521() {
    assert_eq!(closest_to_target(vec![9, 12, 3, 7, 15], 5), 2);
    assert_eq!(
        closest_to_target(vec![1000000, 1000000, 1000000], 1),
        999999
    );
    assert_eq!(closest_to_target(vec![1, 2, 4, 8, 16], 0), 0);
}
