// https://leetcode.com/problems/rectangle-overlap/
pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
    todo!()
}
// math
#[test]
#[ignore]
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
