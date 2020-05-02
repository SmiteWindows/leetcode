// https://leetcode.com/problems/friend-circles/
pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// depth_first_search union_find
#[test]
#[ignore]
fn test2_547() {
    assert_eq!(
        find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
        2
    );
    assert_eq!(
        find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 1], vec![0, 1, 1],]),
        1
    );
}
