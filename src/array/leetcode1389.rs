// https://leetcode.com/problems/create-target-array-in-the-given-order/
pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1389() {
    assert_eq!(
        create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
        vec![0, 4, 1, 3, 2]
    );
    assert_eq!(
        create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]),
        vec![0, 1, 2, 3, 4]
    );
    assert_eq!(create_target_array(vec![1], vec![0]), vec![1]);
}
