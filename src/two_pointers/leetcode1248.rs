// https://leetcode.com/problems/count-number-of-nice-subarrays/
pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    todo!()
}
// two_pointers
#[test]
#[ignore]
fn test1_1248() {
    assert_eq!(number_of_subarrays(vec![1, 1, 2, 1, 1], 3), 2);
    assert_eq!(number_of_subarrays(vec![2, 4, 6], 1), 0);
    assert_eq!(
        number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2),
        16
    );
}
