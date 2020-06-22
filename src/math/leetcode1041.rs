// https://leetcode.com/problems/robot-bounded-in-circle/
// TODO
pub fn is_robot_bounded(instructions: String) -> bool {
    let mut x = 0;
    let mut y = 0;
    let mut i = 0;
    let d = [[1, 0], [0, 1], [-1, 0], [0, -1]];
    for c in instructions.chars() {
        match c {
            'G' => {
                x += d[i][0];
                y += d[i][1];
            }
            'L' => {
                i = (i + 1) % 3;
            }
            'R' => {
                i = (i + 3) % 3;
            }
            _ => (),
        }
    }
    x == 0 && y == 0 || i != 0
}
// math
#[test]
fn test1_1041() {
    assert_eq!(is_robot_bounded(String::from("GGLLGG")), true);
    assert_eq!(is_robot_bounded(String::from("GG")), false);
    assert_eq!(is_robot_bounded(String::from("GL")), true);
}
