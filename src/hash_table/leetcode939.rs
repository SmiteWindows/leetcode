// https://leetcode.com/problems/minimum-area-rectangle/
pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_939() {
    assert_eq!(
        min_area_rect(vec![
            vec![1, 1],
            vec![1, 3],
            vec![3, 1],
            vec![3, 3],
            vec![2, 2]
        ]),
        4
    );
    assert_eq!(
        min_area_rect(vec![
            vec![1, 1],
            vec![1, 3],
            vec![3, 1],
            vec![3, 3],
            vec![4, 1],
            vec![4, 3]
        ]),
        2
    );
}
