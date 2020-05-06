// https://leetcode.com/problems/lemonade-change/
pub fn lemonade_change(bills: Vec<i32>) -> bool {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_860() {
    assert_eq!(lemonade_change(vec![5, 5, 5, 10, 20]), true);
    assert_eq!(lemonade_change(vec![5, 5, 10]), true);
    assert_eq!(lemonade_change(vec![10, 10]), false);
    assert_eq!(lemonade_change(vec![5, 5, 10, 10, 20]), false);
}
