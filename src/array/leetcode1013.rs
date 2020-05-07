// https://leetcode.com/problems/partition-array-into-three-parts-with-equal-sum/
pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1013() {
    assert_eq!(
        can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1]),
        true
    );
    assert_eq!(
        can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1]),
        false
    );
    assert_eq!(
        can_three_parts_equal_sum(vec![3, 3, 6, 5, -2, 2, 5, 1, -9, 4]),
        true
    );
}
