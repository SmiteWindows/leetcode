// https://leetcode.com/problems/jump-game-iii/
pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
    todo!()
}
// graph breadth_first_search
#[test]
#[ignore]
fn test2_1306() {
    assert_eq!(can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5), true);
    assert_eq!(can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0), true);
    assert_eq!(can_reach(vec![3, 0, 2, 1, 2], 2), false);
}
