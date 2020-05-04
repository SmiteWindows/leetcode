// https://leetcode.com/problems/wiggle-sort-ii/
pub fn wiggle_sort(nums: &mut Vec<i32>) {
    todo!()
}
// sort
#[test]
#[ignore]
fn test1_324() {
    let mut nums1 = vec![1, 5, 1, 1, 6, 4];
    wiggle_sort(&mut nums1);
    assert_eq!(nums1, vec![1, 4, 1, 5, 1, 6]);
    let mut nums2 = vec![1, 3, 2, 2, 3, 1];
    wiggle_sort(&mut nums2);
    assert_eq!(nums2, vec![2, 3, 1, 3, 1, 2]);
}
