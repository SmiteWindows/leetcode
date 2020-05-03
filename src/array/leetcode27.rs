// https://leetcode.com/problems/remove-element/
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    todo!()
}
// array two_pointers
#[test]
#[ignore]
fn test2_27() {
    let mut nums1 = vec![3, 2, 2, 3];
    assert_eq!(remove_element(&mut nums1, 3), 2);
    let mut nums2 = vec![0, 1, 2, 2, 3, 0, 4, 2];
    assert_eq!(remove_element(&mut nums2, 2), 5);
}
