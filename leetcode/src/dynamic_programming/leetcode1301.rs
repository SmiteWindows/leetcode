// https://leetcode-cn.com/problems/number-of-paths-with-max-score/
pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1301() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        paths_with_max_score(vec_string!["E23", "2X2", "12S"]),
        vec![7, 1]
    );
    assert_eq!(
        paths_with_max_score(vec_string!["E12", "1X1", "21S"]),
        vec![4, 2]
    );
    assert_eq!(
        paths_with_max_score(vec_string!["E11", "XXX", "11S"]),
        vec![0, 0]
    );
}
