// https://leetcode.com/problems/walking-robot-simulation/
pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_874() {
    assert_eq!(robot_sim(vec![4, -1, 3], vec![vec![]]), 25);
    assert_eq!(robot_sim(vec![4, -1, 4, -2, 4], vec![vec![2, 4]]), 65);
}
