// https://leetcode-cn.com/problems/island-perimeter/
// Runtime: 12 ms
// Memory Usage: 2 MB
pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut sum = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 {
                if i > 0 && grid[i - 1][j] == 0 || i == 0 {
                    sum += 1;
                }
                if i < n - 1 && grid[i + 1][j] == 0 || i == n - 1 {
                    sum += 1;
                }
                if j > 0 && grid[i][j - 1] == 0 || j == 0 {
                    sum += 1;
                }
                if j < m - 1 && grid[i][j + 1] == 0 || j == m - 1 {
                    sum += 1;
                }
            }
        }
    }
    sum
}
// hash_table
#[test]
fn test1_463() {
    use leetcode_prelude::vec2;
    assert_eq!(
        island_perimeter(vec2![
            [0, 1, 0, 0],
            [1, 1, 1, 0],
            [0, 1, 0, 0],
            [1, 1, 0, 0]
        ]),
        16
    );
}
