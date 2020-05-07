// https://leetcode.com/problems/shortest-bridge/
pub fn shortest_bridge(a: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// depth_first_search breadth_first_search
#[test]
#[ignore]
fn test1_934() {
    assert_eq!(shortest_bridge(vec![vec![0, 1], vec![1, 0]]), 1);
    assert_eq!(
        shortest_bridge(vec![vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 1]]),
        2
    );
    assert_eq!(
        shortest_bridge(vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1]
        ]),
        1
    );
}
