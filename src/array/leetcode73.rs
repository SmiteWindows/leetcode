// https://leetcode.com/problems/set-matrix-zeroes/
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_73() {
    let mut nums1 = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    set_zeroes(&mut nums1);
    assert_eq!(nums1, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
    let mut nums2 = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    set_zeroes(&mut nums2);
    assert_eq!(
        nums2,
        vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
    );
}
