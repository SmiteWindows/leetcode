// https://leetcode.com/problems/delete-columns-to-make-sorted/
pub fn min_deletion_size(a: Vec<String>) -> i32 {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_944() {
    assert_eq!(
        min_deletion_size(vec![
            String::from("cba"),
            String::from("daf"),
            String::from("ghi")
        ]),
        1
    );
    assert_eq!(
        min_deletion_size(vec![String::from("a"), String::from("b")]),
        0
    );
    assert_eq!(
        min_deletion_size(vec![
            String::from("zyx"),
            String::from("wvu"),
            String::from("tsr")
        ]),
        3
    );
}
