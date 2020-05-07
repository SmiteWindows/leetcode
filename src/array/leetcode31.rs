// https://leetcode.com/problems/next-permutation/
pub fn next_permutation(nums: &mut Vec<i32>) {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_31() {
    let mut nums1 = vec![1, 2, 3];
    next_permutation(&mut nums1);
    assert_eq!(nums1, vec![1, 3, 2]);
    let mut nums2 = vec![3, 2, 1];
    next_permutation(&mut nums2);
    assert_eq!(nums2, vec![1, 2, 3]);
    let mut nums3 = vec![1, 1, 5];
    next_permutation(&mut nums3);
    assert_eq!(nums3, vec![1, 5, 1]);
}
