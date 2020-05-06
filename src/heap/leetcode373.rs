// https://leetcode.com/problems/find-k-pairs-with-smallest-sums/
pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    todo!()
}
// heap
#[test]
#[ignore]
fn test1_373() {
    assert_eq!(
        k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
        vec![vec![1, 2], vec![1, 4], vec![1, 6]]
    );
    assert_eq!(
        k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
        vec![vec![1, 1], vec![1, 1]]
    );
    assert_eq!(
        k_smallest_pairs(vec![1, 2], vec![3], 3),
        vec![vec![1, 3], vec![2, 3]]
    );
}
