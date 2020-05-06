// https://leetcode.com/problems/delete-columns-to-make-sorted-ii/
pub fn min_deletion_size(a: Vec<String>) -> i32 {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_955() {
    assert_eq!(
        min_deletion_size(vec![
            String::from("ca"),
            String::from("bb"),
            String::from("ac")
        ]),
        1
    );
    assert_eq!(
        min_deletion_size(vec![
            String::from("xc"),
            String::from("yb"),
            String::from("za")
        ]),
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
