// https://leetcode.com/problems/intersection-of-two-arrays/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashSet;
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let h1 = nums1.into_iter().collect::<HashSet<_>>();
    let h2 = nums2.into_iter().collect::<HashSet<_>>();
    let bitand = &h1 & &h2;
    bitand.into_iter().collect()
}
// binary_search two_pointers hash_table sort
#[test]
fn test3_349() {
    assert_eq!(intersection(vec![1, 2, 2, 1], vec![2, 2]), vec![2]);
    let mut a = intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
    a.sort_unstable();
    assert_eq!(a, vec![4, 9]);
}
