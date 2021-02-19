// https://leetcode-cn.com/problems/rotate-array/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let k = k as usize % nums.len();
    nums.rotate_right(k);
}
// array
#[test]
fn test1_189() {
    let mut nums1 = vec![1, 2, 3, 4, 5, 6, 7];
    rotate(&mut nums1, 3);
    assert_eq!(nums1, vec![5, 6, 7, 1, 2, 3, 4]);
    let mut nums2 = vec![-1, -100, 3, 99];
    rotate(&mut nums2, 2);
    assert_eq!(nums2, vec![3, 99, -1, -100]);
}
