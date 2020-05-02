// https://leetcode.com/problems/check-if-it-is-a-straight-line/
pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    todo!()
}
// math array geometry
#[test]
#[ignore]
fn test3_1232() {
    assert_eq!(
        check_straight_line(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7]
        ]),
        true
    );
    assert_eq!(
        check_straight_line(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7]
        ]),
        false
    );
}