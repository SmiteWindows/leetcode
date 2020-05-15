// https://leetcode.com/problems/check-if-it-is-a-straight-line/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    let n = coordinates.len();
    if n == 2 {
        return true;
    }
    let x1 = coordinates[1][0] - coordinates[0][0];
    let y1 = coordinates[1][1] - coordinates[0][1];
    for i in 1..n {
        let xi = coordinates[i][0] - coordinates[0][0];
        let yi = coordinates[i][1] - coordinates[0][1];
        if x1 * yi != xi * y1 {
            return false;
        }
    }
    true
}
// math array geometry
#[test]
fn test2_1232() {
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
