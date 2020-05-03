// https://leetcode.com/problems/perfect-rectangle/
pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
    todo!()
}
// line_sweep
#[test]
#[ignore]
fn test1_391() {
    assert_eq!(
        is_rectangle_cover(vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![3, 2, 4, 4],
            vec![1, 3, 2, 4],
            vec![2, 3, 3, 4]
        ]),
        true
    );
    assert_eq!(
        is_rectangle_cover(vec![
            vec![1, 1, 2, 3],
            vec![1, 3, 2, 4],
            vec![3, 1, 4, 2],
            vec![3, 2, 4, 4]
        ]),
        false
    );
    assert_eq!(
        is_rectangle_cover(vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![1, 3, 2, 4],
            vec![3, 2, 4, 4]
        ]),
        false
    );
    assert_eq!(
        is_rectangle_cover(vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![1, 3, 2, 4],
            vec![2, 2, 4, 4]
        ]),
        false
    );
}
