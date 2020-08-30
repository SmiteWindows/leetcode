// https://leetcode-cn.com/problems/intersection-of-two-arrays/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashSet,
};
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums1 = nums1;
    nums1.sort_unstable();
    let mut nums2 = nums2;
    nums2.sort_unstable();
    let mut hs = HashSet::new();
    let (mut i, mut j) = (0, 0);
    while i < nums1.len() && j < nums2.len() {
        match nums1[i].cmp(&nums2[j]) {
            Less => {
                i += 1;
            }
            Equal => {
                hs.insert(nums1[i]);
                i += 1;
                j += 1;
            }
            Greater => {
                j += 1;
            }
        }
    }
    hs.into_iter().collect()
}
// binary_search two_pointers hash_table sort
#[test]
fn test4_349() {
    use leetcode_prelude::assert_eq_sorted;
    assert_eq!(intersection(vec![1, 2, 2, 1], vec![2, 2]), vec![2]);
    assert_eq_sorted!(intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9]);
}
