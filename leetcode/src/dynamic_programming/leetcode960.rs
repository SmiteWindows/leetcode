// https://leetcode.com/problems/delete-columns-to-make-sorted-iii/
pub fn min_deletion_size(a: Vec<String>) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_960() {
    assert_eq!(
        min_deletion_size(vec![String::from("babca"), String::from("bbazb")]),
        3
    );
    assert_eq!(min_deletion_size(vec![String::from("edcba")]), 4);
    assert_eq!(
        min_deletion_size(vec![
            String::from("ghi"),
            String::from("def"),
            String::from("abc")
        ]),
        0
    );
}
