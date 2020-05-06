// https://leetcode.com/problems/two-city-scheduling/
pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// greedy
#[test]
#[ignore]
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
