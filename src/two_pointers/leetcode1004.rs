// https://leetcode.com/problems/max-consecutive-ones-iii/
pub fn longest_ones(a: Vec<i32>, k: i32) -> i32 {
    todo!()
}
// sliding_window two_pointers
#[test]
#[ignore]
fn test2_1004() {
    assert_eq!(longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2), 6);
    assert_eq!(
        longest_ones(
            vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
            3
        ),
        10
    );
}
