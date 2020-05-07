// https://leetcode.com/problems/coloring-a-border/
pub fn color_border(grid: Vec<Vec<i32>>, r0: i32, c0: i32, color: i32) -> Vec<Vec<i32>> {
    todo!()
}
// depth_first_search
#[test]
#[ignore]
fn test1_1034() {
    assert_eq!(
        color_border(vec![vec![1, 1], vec![1, 2]], 0, 0, 3),
        vec![vec![3, 3], vec![3, 2]]
    );
    assert_eq!(
        color_border(vec![vec![1, 2, 2], vec![2, 3, 2]], 0, 1, 3),
        vec![vec![1, 3, 3], vec![2, 3, 3]]
    );
    assert_eq!(
        color_border(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]], 1, 1, 2),
        vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]
    );
}
