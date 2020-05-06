// https://leetcode.com/problems/dice-roll-simulation/
pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1223() {
    assert_eq!(die_simulator(2, vec![1, 1, 2, 2, 2, 3]), 34);
    assert_eq!(die_simulator(2, vec![1, 1, 1, 1, 1, 1]), 30);
    assert_eq!(die_simulator(3, vec![1, 1, 1, 2, 2, 3]), 181);
}
