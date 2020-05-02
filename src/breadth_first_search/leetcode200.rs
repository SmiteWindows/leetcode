// https://leetcode.com/problems/number-of-islands/
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    todo!()
}
// depth_first_search breadth_first_search union_find
#[test]
#[ignore]
fn test2_200() {
    assert_eq!(
        num_islands(vec![vec![11110], vec![11010], vec![11000], vec![00000]]),
        1
    );
    assert_eq!(
        num_islands(vec![vec![11000], vec![11000], vec![00100], vec![00011]]),
        3
    );
}
