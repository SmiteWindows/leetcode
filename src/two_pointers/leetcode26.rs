// https://leetcode.com/problems/remove-duplicates-from-sorted-array/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    let mut last = nums[0];
    let mut size = 1;
    for i in 1..n {
        if nums[i] != last {
            last = nums[i];
            nums[size] = nums[i];
            size += 1;
        }
    }
    size as i32
}
// array two_pointers
#[test]
fn test1_26() {
    let mut nums1 = vec![1, 1, 2];
    assert_eq!(remove_duplicates(&mut nums1), 2);
    assert_eq!(&nums1[..2], &vec![1, 2][..]);
    let mut nums2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(remove_duplicates(&mut nums2), 5);
    assert_eq!(&nums2[..5], &vec![0, 1, 2, 3, 4][..])
}
