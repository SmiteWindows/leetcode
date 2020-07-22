// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut j = m;
    let mut res = 0;
    for row in grid.iter().take(n) {
        while j > 0 && row[j - 1] < 0 {
            j -= 1;
        }
        res += m - j;
    }
    res as i32
}
// binary_search array
#[test]
fn test2_1351() {
    assert_eq!(
        count_negatives(vec![
            vec![4, 3, 2, -1],
            vec![3, 2, 1, -1],
            vec![1, 1, -1, -2],
            vec![-1, -1, -2, -3]
        ]),
        8
    );
    assert_eq!(count_negatives(vec![vec![3, 2], vec![1, 0]]), 0);
    assert_eq!(count_negatives(vec![vec![1, -1], vec![-1, -1]]), 3);
    assert_eq!(count_negatives(vec![vec![-1]]), 1);
}
