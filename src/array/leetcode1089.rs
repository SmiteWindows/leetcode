// https://leetcode.com/problems/duplicate-zeros/
pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1089() {
    let mut arr1 = vec![1, 0, 2, 3, 0, 4, 5, 0];
    duplicate_zeros(&mut arr1);
    assert_eq!(arr1, vec![1, 0, 0, 2, 3, 0, 0, 4]);
    let mut arr2 = vec![1, 2, 3];
    duplicate_zeros(&mut arr2);
    assert_eq!(arr2, vec![1, 2, 3]);
}
