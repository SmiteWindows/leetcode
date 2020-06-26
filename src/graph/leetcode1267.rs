// https://leetcode.com/problems/count-servers-that-communicate/
// Runtime: 12 ms
// Memory Usage: 2.4 MB
pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut rows = vec![0; n];
    let mut cols = vec![0; m];
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 {
                rows[i] += 1;
                cols[j] += 1;
            }
        }
    }
    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 && (rows[i] > 1 || cols[j] > 1) {
                res += 1;
            }
        }
    }
    res
}
// graph array
#[test]
fn test1_1267() {
    assert_eq!(count_servers(vec![vec![1, 0], vec![0, 1]]), 0);
    assert_eq!(count_servers(vec![vec![1, 0], vec![1, 1]]), 3);
    assert_eq!(
        count_servers(vec![
            vec![1, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1]
        ]),
        4
    );
}
