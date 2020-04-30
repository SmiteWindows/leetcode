// https://leetcode.com/problems/self-crossing/
pub fn is_self_crossing(x: Vec<i32>) -> bool {
    todo!()
}
// math
#[test]
fn test1_335() {
    assert_eq!(is_self_crossing(vec![2, 1, 1, 2]), true);
    assert_eq!(is_self_crossing(vec![1, 2, 3, 4]), false);
    assert_eq!(is_self_crossing(vec![1, 1, 1, 1]), true);
}
