// https://leetcode-cn.com/problems/two-city-scheduling/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
    let n = costs.len();
    let mut diffs = costs.iter().map(|v| v[0] - v[1]).collect::<Vec<_>>();
    diffs.sort_unstable();
    let sum_of_b = costs.iter().map(|v| v[1]).sum::<i32>();
    let sum_of_diff = diffs.iter().take(n / 2).sum::<i32>();
    sum_of_b + sum_of_diff
}
// greedy
#[test]
fn test1_1029() {
    use leetcode_prelude::vec2;
    assert_eq!(
        two_city_sched_cost(vec2![[10, 20], [30, 200], [400, 50], [30, 20]]),
        110
    );
}
