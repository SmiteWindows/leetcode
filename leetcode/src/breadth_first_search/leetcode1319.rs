// https://leetcode.com/problems/number-of-operations-to-make-network-connected/
pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// union_find depth_first_search breadth_first_search
#[test]
#[ignore]
fn test2_1319() {
    assert_eq!(
        make_connected(4, vec![vec![0, 1], vec![0, 2], vec![1, 2]]),
        1
    );
    assert_eq!(
        make_connected(
            6,
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]]
        ),
        2
    );
    assert_eq!(
        make_connected(6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]]),
        -1
    );
    assert_eq!(
        make_connected(5, vec![vec![0, 1], vec![0, 2], vec![3, 4], vec![2, 3]]),
        0
    );
}
