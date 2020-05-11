// https://leetcode.com/problems/median-of-two-sorted-arrays/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let m = nums1.len();
    let n = nums2.len();
    if m > n {
        return find_median_sorted_arrays(nums2, nums1);
    }
    let mut i_min = 0;
    let mut i_max = m;
    let half_len = (m + n + 1) / 2;
    while i_min <= i_max {
        let i = (i_max + i_min) / 2;
        let j = half_len - i;
        if i < i_max && nums2[j - 1] > nums1[i] {
            i_min = i + 1;
        } else if i > i_min && nums1[i - 1] > nums2[j] {
            i_max = i - 1;
        } else {
            let max_left = if i == 0 {
                nums2[j - 1]
            } else if j == 0 {
                nums1[i - 1]
            } else {
                i32::max(nums1[i - 1], nums2[j - 1])
            };
            if (m + n) % 2 == 1 {
                return max_left.into();
            }
            let min_right = if i == m {
                nums2[j]
            } else if j == n {
                nums1[i]
            } else {
                i32::min(nums2[j], nums1[i])
            };
            return (max_left + min_right) as f64 / 2.0;
        }
    }
    0.0
}
// array binary_search divide_and_conquer
#[test]
fn test1_4() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    let res = 2.0;
    assert_eq!(res, find_median_sorted_arrays(nums1, nums2));
    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    let res = 2.5;
    assert_eq!(res, find_median_sorted_arrays(nums1, nums2));
}
