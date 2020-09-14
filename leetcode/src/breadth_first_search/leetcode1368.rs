// https://leetcode-cn.com/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid/
pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// breadth_first_search
#[test]
#[ignore]
fn test1_1368() {
    use leetcode_prelude::vec2;
    assert_eq!(
        min_cost(vec2![
            [1, 1, 1, 1],
            [2, 2, 2, 2],
            [1, 1, 1, 1],
            [2, 2, 2, 2]
        ]),
        3
    );
    assert_eq!(min_cost(vec2![[1, 1, 3], [3, 2, 2], [1, 1, 4]]), 0);
    assert_eq!(min_cost(vec2![[1, 2], [4, 3]]), 1);
    assert_eq!(min_cost(vec2![[2, 2, 2], [2, 2, 2]]), 3);
    assert_eq!(min_cost(vec2![[4]]), 0);
}
