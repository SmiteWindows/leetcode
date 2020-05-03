// https://leetcode.com/problems/subarrays-with-k-different-integers/
pub fn subarrays_with_k_distinct(a: Vec<i32>, k: i32) -> i32 {
    todo!()
}
// sliding_window two_pointers hash_table
#[test]
#[ignore]
fn test1_992() {
    assert_eq!(subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2), 7);
    assert_eq!(subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3), 3);
}
