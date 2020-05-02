// https://leetcode.com/problems/robot-bounded-in-circle/
pub fn is_robot_bounded(instructions: String) -> bool {
    todo!()
}
// math
#[test]
#[ignore]
fn test1_1041() {
    assert_eq!(is_robot_bounded(String::from("GGLLGG")), true);
    assert_eq!(
        is_robot_bounded(String::from("GG")),
        false
    );
    assert_eq!(is_robot_bounded(String::from("GL")), true);
}