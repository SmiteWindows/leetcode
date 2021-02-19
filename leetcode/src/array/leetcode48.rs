// https://leetcode-cn.com/problems/rotate-image/
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
    use leetcode_prelude::vec2;
    let mut nums1 = vec2![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    rotate(&mut nums1);
    assert_eq!(nums1, vec2![[7, 4, 1], [8, 5, 2], [9, 6, 3]]);
    let mut nums2 = vec2![
        [5, 1, 9, 11],
        [2, 4, 8, 10],
        [13, 3, 6, 7],
        [15, 14, 12, 16]
    ];
    rotate(&mut nums2);
    assert_eq!(
        nums2,
        vec2![
            [15, 13, 2, 5],
            [14, 3, 4, 1],
            [12, 6, 8, 9],
            [16, 7, 10, 11]
        ]
    );
    let mut nums3 = vec2![[1]];
    rotate(&mut nums3);
    assert_eq!(nums3, vec2![[1]]);
    let mut nums4 = vec2![[1, 2], [3, 4]];
    rotate(&mut nums4);
    assert_eq!(nums4, vec2![[3, 1], [4, 2]]);
}
