// https://leetcode.com/problems/minimum-area-rectangle-ii/
pub fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
    todo!()
}
// math geometry
#[test]
#[ignore]
fn test1_963() {
    assert_eq!(
        min_area_free_rect(vec![vec![1, 2], vec![2, 1], vec![1, 0], vec![0, 1]]),
        2.00000
    );
    assert_eq!(
        min_area_free_rect(vec![
            vec![0, 1],
            vec![2, 1],
            vec![1, 1],
            vec![1, 0],
            vec![2, 0]
        ]),
        1.00000
    );
    assert_eq!(
        min_area_free_rect(vec![
            vec![0, 3],
            vec![1, 2],
            vec![3, 1],
            vec![1, 3],
            vec![2, 1]
        ]),
        0.00000
    );
    assert_eq!(
        min_area_free_rect(vec![
            vec![3, 1],
            vec![1, 1],
            vec![0, 1],
            vec![2, 1],
            vec![3, 3],
            vec![3, 2],
            vec![0, 2],
            vec![2, 3]
        ]),
        2.00000
    );
}
