// https://leetcode.com/problems/unique-paths-iii/
pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// backtracking depth_first_search
#[test]
#[ignore]
fn test1_980() {
    assert_eq!(
        unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]]),
        2
    );
    assert_eq!(
        unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]]),
        4
    );
    assert_eq!(unique_paths_iii(vec![vec![0, 1], vec![2, 0]]), 0);
}
