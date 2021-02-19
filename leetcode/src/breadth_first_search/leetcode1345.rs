// https://leetcode-cn.com/problems/jump-game-iv/
pub fn min_jumps(arr: Vec<i32>) -> i32 {
    todo!()
}
// breadth_first_search
#[test]
#[ignore]
fn test1_1345() {
    assert_eq!(
        min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]),
        3
    );
    assert_eq!(min_jumps(vec![7]), 0);
    assert_eq!(min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]), 1);
    assert_eq!(min_jumps(vec![6, 1, 9]), 2);
    assert_eq!(min_jumps(vec![11, 22, 7, 7, 7, 7, 7, 7, 7, 22, 13]), 3);
}
