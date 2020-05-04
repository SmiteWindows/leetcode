// https://leetcode.com/problems/as-far-from-land-as-possible/
pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// graph breadth_first_search
#[test]
#[ignore]
fn test2_1162() {
    assert_eq!(
        max_distance(vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]),
        2
    );
    assert_eq!(
        max_distance(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]),
        4
    );
}