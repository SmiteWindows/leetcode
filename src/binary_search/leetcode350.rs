// https://leetcode.com/problems/intersection-of-two-arrays-ii/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    if nums1.len() > nums2.len() {
        return intersect(nums2, nums1);
    }
    let mut nums1 = nums1;
    nums1.sort_unstable();
    let mut nums2 = nums2;
    nums2.sort_unstable();
    let n = nums1.len();
    let mut res = vec![];
    let mut k = 0;
    for i in nums2 {
        if k == n {
            break;
        }
        let mut left = k;
        let mut right = n - 1;
        while left < right {
            let mid = left + (right - left) / 2;
            if nums1[mid] < i {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if nums1[left] == i {
            k = left + 1;
            res.push(i);
        }
    }
    res
}
// binary_search two_pointers hash_table sort
#[test]
fn test2_350() {
    assert_eq!(intersect(vec![1, 2, 2, 1], vec![2, 2]), vec![2, 2]);
    assert_eq!(intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9]);
}
