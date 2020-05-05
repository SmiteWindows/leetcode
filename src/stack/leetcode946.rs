// https://leetcode.com/problems/validate-stack-sequences/
pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    todo!()
}
// stack
#[test]
#[ignore]
fn test1_946() {
    assert_eq!(
        validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1]),
        true
    );
    assert_eq!(
        validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 3, 5, 1, 2]),
        false
    );
}
