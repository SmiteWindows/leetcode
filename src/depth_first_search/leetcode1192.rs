// https://leetcode.com/problems/critical-connections-in-a-network/
pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    todo!()
}
// depth_first_search
#[test]
#[ignore]
fn test1_1192() {
    assert_eq!(
        critical_connections(4, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]]),
        vec![vec![1, 3]]
    );
}
