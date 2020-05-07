//https://leetcode.com/problems/add-to-array-form-of-integer/
pub fn add_to_array_form(a: Vec<i32>, k: i32) -> Vec<i32> {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_989() {
    assert_eq!(add_to_array_form(vec![1, 2, 0, 0], 34), vec![1, 2, 3, 4]);
    assert_eq!(add_to_array_form(vec![2, 7, 4], 181), vec![4, 5, 5]);
    assert_eq!(add_to_array_form(vec![2, 1, 5], 806), vec![1, 0, 2, 1]);
    assert_eq!(
        add_to_array_form(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 1),
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}
