// https://leetcode.com/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid/
pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// breadth_first_search
#[test]
#[ignore]
fn test1_1368() {
    assert_eq!(
        min_cost(vec![
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2]
        ]),
        3
    );
    assert_eq!(
        min_cost(vec![vec![1, 1, 3], vec![3, 2, 2], vec![1, 1, 4]]),
        0
    );
    assert_eq!(min_cost(vec![vec![1, 2], vec![4, 3]]), 1);
    assert_eq!(min_cost(vec![vec![2, 2, 2], vec![2, 2, 2]]), 3);
    assert_eq!(min_cost(vec![vec![4]]), 0);
}
