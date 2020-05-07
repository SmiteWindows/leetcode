// https://leetcode.com/problems/rotting-oranges/
pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// breadth_first_search
#[test]
#[ignore]
fn test1_994() {
    assert_eq!(
        oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]),
        4
    );
    assert_eq!(
        oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]),
        -1
    );
    assert_eq!(oranges_rotting(vec![vec![0, 2]]), 0);
}
