// https://leetcode.com/problems/sliding-puzzle/
pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// breadth_first_search
#[test]
#[ignore]
fn test1_773() {
    assert_eq!(sliding_puzzle(vec![vec![1, 2, 3], vec![4, 0, 5]]), 1);
    assert_eq!(sliding_puzzle(vec![vec![1, 2, 3], vec![5, 4, 0]]), -1);
    assert_eq!(sliding_puzzle(vec![vec![4, 1, 2], vec![5, 0, 3]]), 5);
    assert_eq!(sliding_puzzle(vec![vec![3, 2, 4], vec![1, 5, 0]]), 14);
}
