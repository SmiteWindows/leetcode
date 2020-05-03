// https://leetcode.com/problems/sort-colors/
pub fn sort_colors(nums: &mut Vec<i32>) {
    todo!()
}
// array two_pointers sort
#[test]
#[ignore]
fn test2_75() {
    let mut nums1 = vec![2, 0, 2, 1, 1, 0];
    sort_colors(&mut nums1);
    assert_eq!(nums1, vec![0, 0, 1, 1, 2, 2]);
}
