// https://leetcode-cn.com/problems/move-zeroes/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut j = 0;
    for i in 0..nums.len() {
        let x = nums[i];
        if x != 0 {
            nums[i] = 0;
            nums[j] = x;
            j += 1;
        }
    }
}
// array two_pointers
#[test]
fn test2_283() {
    let mut nums1 = vec![0, 1, 0, 3, 12];
    move_zeroes(&mut nums1);
    assert_eq!(nums1, vec![1, 3, 12, 0, 0]);
}
