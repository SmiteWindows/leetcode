// https://leetcode.com/problems/intersection-of-two-arrays/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashSet;
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums1 = nums1;
    nums1.sort_unstable();
    let mut nums2 = nums2;
    nums2.sort_unstable();
    let mut hs = HashSet::new();
    for i in nums2 {
        if !hs.contains(&i) && nums1.binary_search(&i).is_ok() {
            hs.insert(i);
        }
    }
    hs.into_iter().collect()
}
// binary_search two_pointers hash_table sort
#[test]
fn test2_349() {
    assert_eq!(intersection(vec![1, 2, 2, 1], vec![2, 2]), vec![2]);
    let mut a = intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
    a.sort_unstable();
    assert_eq!(a, vec![4, 9]);
}
