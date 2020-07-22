// https://leetcode.com/problems/wiggle-sort-ii/
// Runtime: 8 ms
// Memory Usage: 2.1 MB
pub fn wiggle_sort(nums: &mut Vec<i32>) {
    let n = nums.len();
    let mut sorted = nums.to_vec();
    sorted.sort_unstable();
    let k = if n % 2 == 0 { n / 2 } else { n / 2 + 1 };
    for i in 0..k {
        nums[i * 2] = sorted[k - 1 - i];
    }
    for i in 0..(n - k) {
        nums[i * 2 + 1] = sorted[n - 1 - i];
    }
}
// sort
#[test]
fn test1_324() {
    let mut nums1 = vec![1, 5, 1, 1, 6, 4];
    wiggle_sort(&mut nums1);
    assert_eq!(nums1, vec![1, 6, 1, 5, 1, 4]);
    let mut nums2 = vec![1, 3, 2, 2, 3, 1];
    wiggle_sort(&mut nums2);
    assert_eq!(nums2, vec![2, 3, 1, 3, 1, 2]);
}
