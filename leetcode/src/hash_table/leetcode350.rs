// https://leetcode-cn.com/problems/intersection-of-two-arrays-ii/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn intersect(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    if nums1.len() > nums2.len() {
        return intersect(nums2, nums1);
    }
    let mut hm: HashMap<i32, i32> = HashMap::new();
    for &n in &nums1 {
        *hm.entry(n).or_default() += 1;
    }
    let mut k = 0;
    for &n in &nums2 {
        let count = hm.entry(n).or_default();
        if *count > 0 {
            nums1[k] = n;
            k += 1;
            *count -= 1;
        }
    }
    nums1[..k].to_vec()
}
// binary_search two_pointers hash_table sort
#[test]
fn test3_350() {
    use leetcode_prelude::assert_eq_sorted;
    assert_eq!(intersect(vec![1, 2, 2, 1], vec![2, 2]), vec![2, 2]);
    assert_eq_sorted!(intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9]);
}
