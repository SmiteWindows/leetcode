// https://leetcode-cn.com/problems/remove-element/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let n = nums.len();
    let mut size = 0_usize;
    for i in 0..n {
        if nums[i] != val {
            nums[size] = nums[i];
            size += 1;
        }
    }
    size as i32
}
// array two_pointers
#[test]
fn test1_27() {
    let mut nums1 = vec![3, 2, 2, 3];
    assert_eq!(remove_element(&mut nums1, 3), 2);
    assert_eq!(&nums1[..2], vec![2, 2].as_slice());
    let mut nums2 = vec![0, 1, 2, 2, 3, 0, 4, 2];
    assert_eq!(remove_element(&mut nums2, 2), 5);
    assert_eq!(&nums2[..5], vec![0, 1, 3, 0, 4].as_slice());
}
