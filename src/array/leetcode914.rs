// https://leetcode.com/problems/x-of-a-kind-in-a-deck-of-cards/
pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
    todo!()
}
// math array
#[test]
#[ignore]
fn test2_914() {
    assert_eq!(has_groups_size_x(vec![1, 2, 3, 4, 4, 3, 2, 1]), true);
    assert_eq!(has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]), false);
    assert_eq!(has_groups_size_x(vec![1]), false);
    assert_eq!(has_groups_size_x(vec![1, 1]), true);
    assert_eq!(has_groups_size_x(vec![1, 1, 2, 2, 2, 2]), true);
}
