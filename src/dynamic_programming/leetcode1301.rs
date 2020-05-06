// https://leetcode.com/problems/number-of-paths-with-max-score/
pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1301() {
    assert_eq!(
        paths_with_max_score(vec![
            String::from("E23"),
            String::from("2X2"),
            String::from("12S")
        ]),
        vec![7, 1]
    );
    assert_eq!(
        paths_with_max_score(vec![
            String::from("E12"),
            String::from("1X1"),
            String::from("21S")
        ]),
        vec![4, 2]
    );
    assert_eq!(
        paths_with_max_score(vec![
            String::from("E11"),
            String::from("XXX"),
            String::from("11S")
        ]),
        vec![0, 0]
    );
}
