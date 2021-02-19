// https://leetcode-cn.com/problems/most-stones-removed-with-same-row-or-column/
pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// union_find depth_first_search
#[test]
#[ignore]
fn test2_947() {
    use leetcode_prelude::vec2;
    assert_eq!(
        remove_stones(vec2![[0, 0], [0, 1], [1, 0], [1, 2], [2, 1], [2, 2]]),
        5
    );
    assert_eq!(
        remove_stones(vec2![[0, 0], [0, 2], [1, 1], [2, 0], [2, 2]]),
        3
    );
    assert_eq!(remove_stones(vec2![[0, 0]]), 0);
}
