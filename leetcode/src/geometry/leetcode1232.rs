// https://leetcode-cn.com/problems/check-if-it-is-a-straight-line/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
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
fn test3_1232() {
    use leetcode_prelude::vec2;
    assert_eq!(
        check_straight_line(vec2![[1, 2], [2, 3], [3, 4], [4, 5], [5, 6], [6, 7]]),
        true
    );
    assert_eq!(
        check_straight_line(vec2![[1, 1], [2, 2], [3, 4], [4, 5], [5, 6], [7, 7]]),
        false
    );
}
