// https://leetcode.com/problems/remove-duplicates-from-sorted-array/
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    todo!()
}
// array two_pointers
#[test]
#[ignore]
fn test2_26() {
    let mut nums1 = vec![1, 1, 2];
    assert_eq!(remove_duplicates(&mut nums1), 2);
    let mut nums2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(remove_duplicates(&mut nums2), 5);
}
