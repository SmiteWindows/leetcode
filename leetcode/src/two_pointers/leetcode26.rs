// https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array/
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut i = 0;
    for j in 1..nums.len() {
        if nums[i] != nums[j] {
            i += 1;
            nums[i] = nums[j];
        }
    }
    i as i32 + 1
}
// Runtime: 4 ms
// Memory Usage: 2.2 MB
// ✔
// array two_pointers
#[test]
fn test1_26() {
    let mut nums1 = vec![1, 1, 2];
    assert_eq!(remove_duplicates(&mut nums1), 2);
    assert_eq!(&nums1[..2], vec![1, 2].as_slice());
    let mut nums2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(remove_duplicates(&mut nums2), 5);
    assert_eq!(&nums2[..5], vec![0, 1, 2, 3, 4].as_slice());
}
