// https://leetcode.com/problems/intersection-of-two-arrays/
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    todo!()
}
// binary_search two_pointers hash_table sort
#[test]
#[ignore]
fn test4_349() {
    assert_eq!(intersection(vec![1, 2, 2, 1], vec![2, 2]), vec![2]);
    let mut a = intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
    a.sort_unstable();
    assert_eq!(a, vec![4, 9]);
}
