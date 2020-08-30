// https://leetcode-cn.com/problems/min-cost-climbing-stairs/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut cost = cost;
    let n = cost.len();
    cost.push(0);
    if n < 2 {
        return 0;
    }
    for i in 2..=n {
        cost[i] += cost[i - 1].min(cost[i - 2]);
    }
    cost[n]
}
// dynamic_programming array
#[test]
fn test2_746() {
    assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    assert_eq!(
        min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
        6
    );
}
