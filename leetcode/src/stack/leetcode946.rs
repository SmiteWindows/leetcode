// https://leetcode-cn.com/problems/validate-stack-sequences/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    let mut stack = vec![];
    let mut it = popped.iter().peekable();
    for x in pushed {
        stack.push(x);
        while let (Some(&a), Some(&&b)) = (stack.last(), it.peek()) {
            if a == b {
                stack.pop();
                it.next();
            } else {
                break;
            }
        }
    }
    stack.is_empty()
}
// stack
#[test]
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
