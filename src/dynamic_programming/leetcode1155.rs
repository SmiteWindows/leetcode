// https://leetcode.com/problems/number-of-dice-rolls-with-target-sum/
pub fn num_rolls_to_target(d: i32, f: i32, target: i32) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1155() {
    assert_eq!(num_rolls_to_target(1, 6, 3), 1);
    assert_eq!(num_rolls_to_target(2, 6, 7), 6);
    assert_eq!(num_rolls_to_target(2, 5, 10), 1);
    assert_eq!(num_rolls_to_target(1, 2, 3), 0);
    assert_eq!(num_rolls_to_target(30, 30, 500), 222616187);
}
