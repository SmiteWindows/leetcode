// https://leetcode.com/problems/move-zeroes/
pub fn move_zeroes(nums: &mut Vec<i32>) {
    todo!()
}
// array two_pointers
#[test]
#[ignore]
fn test1_283() {
    let mut nums1 = vec![0, 1, 0, 3, 12];
    move_zeroes(&mut nums1);
    assert_eq!(nums1, vec![1, 3, 12, 0, 0]);
}
