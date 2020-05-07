// https://leetcode.com/problems/summary-ranges/
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_228() {
    assert_eq!(
        summary_ranges(vec![0, 1, 2, 4, 5, 7]),
        vec![
            String::from("0->2"),
            String::from("4->5"),
            String::from("7")
        ]
    );
    assert_eq!(
        summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
        vec![
            String::from("0"),
            String::from("2->4"),
            String::from("6"),
            String::from("8->9")
        ]
    );
}
