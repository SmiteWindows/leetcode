// https://leetcode.com/problems/top-k-frequent-elements/
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    todo!()
}
// hash_table heap
#[test]
#[ignore]
fn test1_347() {
    assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
}
