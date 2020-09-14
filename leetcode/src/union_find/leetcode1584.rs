// https://leetcode-cn.com/problems/min-cost-to-connect-all-points/
pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// union_find
#[test]
#[ignore]
fn test1_1584() {
    use leetcode_prelude::vec2;
    assert_eq!(
        min_cost_connect_points(vec2![[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]]),
        20
    );
    assert_eq!(
        min_cost_connect_points(vec2![[3, 12], [-2, 5], [-4, 1]]),
        18
    );
    assert_eq!(
        min_cost_connect_points(vec2![[0, 0], [1, 1], [1, 0], [-1, 1]]),
        4
    );
    assert_eq!(
        min_cost_connect_points(vec2![[-1000000, -1000000], [1000000, 1000000]]),
        4000000
    );
    assert_eq!(min_cost_connect_points(vec2![[0, 0]]), 0);
}
