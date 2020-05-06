// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/
pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    todo!()
}
// hash_table array
#[test]
#[ignore]
fn test2_1365() {
    assert_eq!(
        smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
        vec![4, 0, 1, 1, 3]
    );
    assert_eq!(
        smaller_numbers_than_current(vec![6, 5, 4, 8]),
        vec![2, 1, 0, 3]
    );
    assert_eq!(
        smaller_numbers_than_current(vec![7, 7, 7, 7]),
        vec![0, 0, 0, 0]
    );
}
