// https://leetcode.com/problems/two-city-scheduling/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
    let n = costs.len();
    let mut diffs: Vec<i32> = costs.iter().map(|v| v[0] - v[1]).collect();
    diffs.sort_unstable();
    let sum_of_b = costs.iter().fold(0, |sum, v| sum + v[1]);
    let sum_of_diff: i32 = diffs.iter().take(n / 2).sum();
    sum_of_b + sum_of_diff
}
// greedy
#[test]
fn test1_1029() {
    assert_eq!(
        two_city_sched_cost(vec![
            vec![10, 20],
            vec![30, 200],
            vec![400, 50],
            vec![30, 20]
        ]),
        110
    );
}
