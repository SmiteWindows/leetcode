// https://leetcode.com/problems/rotate-image/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    matrix.reverse();
    for i in 1..matrix.len() {
        let (left, right) = matrix.split_at_mut(i);
        for (j, left_item) in left.iter_mut().enumerate().take(i) {
            std::mem::swap(&mut left_item[i], &mut right[0][j]);
        }
    }
}
// array
#[test]
fn test1_48() {
    let mut nums1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    rotate(&mut nums1);
    assert_eq!(nums1, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    let mut nums2 = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    rotate(&mut nums2);
    assert_eq!(
        nums2,
        vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11]
        ]
    );
}
