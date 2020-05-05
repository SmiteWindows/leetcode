// https://leetcode.com/problems/baseball-game/
pub fn cal_points(ops: Vec<String>) -> i32 {
    todo!()
}
// stack
#[test]
#[ignore]
fn test1_682() {
    assert_eq!(
        cal_points(vec![
            String::from("5"),
            String::from("2"),
            String::from("C"),
            String::from("D"),
            String::from("+")
        ]),
        30
    );
    assert_eq!(
        cal_points(vec![
            String::from("5"),
            String::from("-2"),
            String::from("4"),
            String::from("C"),
            String::from("D"),
            String::from("9"),
            String::from("+"),
            String::from("+")
        ]),
        27
    );
}
