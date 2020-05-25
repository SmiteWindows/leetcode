// https://leetcode.com/problems/intersection-of-two-arrays-ii/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::cmp::Ordering::{Equal, Greater, Less};
pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums1 = nums1;
    nums1.sort_unstable();
    let mut nums2 = nums2;
    nums2.sort_unstable();
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < nums1.len() && j < nums2.len() {
        match nums1[i].cmp(&nums2[j]) {
            Less => {
                i += 1;
            }
            Equal => {
                nums1[k] = nums1[i];
                i += 1;
                j += 1;
                k += 1;
            }
            Greater => {
                j += 1;
            }
        }
    }
    nums1[..k].to_vec()
}
// binary_search two_pointers hash_table sort
#[test]
fn test1_350() {
    assert_eq!(intersect(vec![1, 2, 2, 1], vec![2, 2]), vec![2, 2]);
    assert_eq!(intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9]);
}
