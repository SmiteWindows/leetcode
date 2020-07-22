// https://leetcode.com/problems/rectangle-overlap/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
    rec1[2] > rec2[0] && rec1[0] < rec2[2] && rec1[1] < rec2[3] && rec1[3] > rec2[1]
}
// math
#[test]
fn test1_836() {
    assert_eq!(
        is_rectangle_overlap(vec![0, 0, 2, 2], vec![1, 1, 3, 3]),
        true
    );
    assert_eq!(
        is_rectangle_overlap(vec![0, 0, 1, 1], vec![1, 0, 2, 1]),
        false
    );
}
