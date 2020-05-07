// https://leetcode.com/problems/positions-of-large-groups/
pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_830() {
    assert_eq!(
        large_group_positions(String::from("abbxxxxzzy")),
        vec![vec![3, 6]]
    );
    assert_eq!(large_group_positions(String::from("abc")), vec![vec![]]);
    assert_eq!(
        large_group_positions(String::from("abcdddeeeeaabbbcd")),
        vec![vec![3, 5], vec![6, 9], vec![12, 14]]
    );
}
