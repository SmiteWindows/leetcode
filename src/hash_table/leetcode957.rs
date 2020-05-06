// https://leetcode.com/problems/prison-cells-after-n-days/
pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_957() {
    assert_eq!(
        prison_after_n_days(vec![0, 1, 0, 1, 1, 0, 0, 1], 7),
        vec![0, 0, 1, 1, 0, 0, 0, 0]
    );
    assert_eq!(
        prison_after_n_days(vec![1, 0, 0, 1, 0, 0, 1, 0], 1000000000),
        vec![0, 0, 1, 1, 1, 1, 1, 0]
    );
}
