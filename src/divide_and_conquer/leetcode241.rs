// https://leetcode.com/problems/different-ways-to-add-parentheses/
pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
    todo!()
}
// divide_and_conquer
#[test]
#[ignore]
fn test1_241() {
    assert_eq!(diff_ways_to_compute(String::from("2-1-1")), vec![0, 2]);
    assert_eq!(
        diff_ways_to_compute(String::from("2*3-4*5")),
        vec![-34, -14, -10, -10, 10]
    );
}
