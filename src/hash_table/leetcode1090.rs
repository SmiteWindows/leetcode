// https://leetcode.com/problems/largest-values-from-labels/
pub fn largest_vals_from_labels(
    values: Vec<i32>,
    labels: Vec<i32>,
    num_wanted: i32,
    use_limit: i32,
) -> i32 {
    todo!()
}
// hash_table greedy
#[test]
#[ignore]
fn test1_1090() {
    assert_eq!(
        largest_vals_from_labels(vec![5, 4, 3, 2, 1], vec![1, 1, 2, 2, 3], 3, 1),
        9
    );
    assert_eq!(
        largest_vals_from_labels(vec![5, 4, 3, 2, 1], vec![1, 3, 3, 3, 2], 3, 2),
        12
    );
    assert_eq!(
        largest_vals_from_labels(vec![9, 8, 8, 7, 6], vec![0, 0, 0, 1, 1], 3, 1),
        16
    );
    assert_eq!(
        largest_vals_from_labels(vec![9, 8, 8, 7, 6], vec![0, 0, 0, 1, 1], 3, 2),
        24
    );
}
