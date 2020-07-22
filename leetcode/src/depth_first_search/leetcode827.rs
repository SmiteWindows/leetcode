// https://leetcode.com/problems/making-a-large-island/
pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// depth_first_search
#[test]
#[ignore]
fn test1_827() {
    assert_eq!(largest_island(vec![vec![1, 0], vec![0, 1]]), 3);
    assert_eq!(largest_island(vec![vec![1, 1], vec![1, 0]]), 4);
    assert_eq!(largest_island(vec![vec![1, 1], vec![1, 1]]), 4);
}
