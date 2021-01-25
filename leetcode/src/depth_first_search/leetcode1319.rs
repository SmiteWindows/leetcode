// https://leetcode-cn.com/problems/number-of-operations-to-make-network-connected/
pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// union_find depth_first_search breadth_first_search
#[test]
#[ignore]
fn test3_1319() {
    use leetcode_prelude::vec2;
    assert_eq!(make_connected(4, vec2![[0, 1], [0, 2], [1, 2]]), 1);
    assert_eq!(
        make_connected(6, vec2![[0, 1], [0, 2], [0, 3], [1, 2], [1, 3]]),
        2
    );
    assert_eq!(make_connected(6, vec2![[0, 1], [0, 2], [0, 3], [1, 2]]), -1);
    assert_eq!(make_connected(5, vec2![[0, 1], [0, 2], [3, 4], [2, 3]]), 0);
}
