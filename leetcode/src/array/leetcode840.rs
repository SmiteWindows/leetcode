// https://leetcode-cn.com/problems/magic-squares-in-grid/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    if n < 3 || m < 3 {
        return 0;
    }
    let mut sum = 0;
    for i in 0..=(n - 3) {
        for j in 0..=(m - 3) {
            if is_magic(&grid, i, j) {
                sum += 1;
            }
        }
    }
    sum
}

fn is_magic(grid: &[Vec<i32>], r: usize, c: usize) -> bool {
    let mut xor = 0;
    for i in 1..10 {
        xor ^= i;
    }
    for i in 0..3 {
        for j in 0..3 {
            xor ^= grid[r + i][c + j];
        }
    }
    if xor != 0 {
        return false;
    }
    let r0 = grid[r][c] + grid[r][c + 1] + grid[r][c + 2];
    if r0 != 15 {
        return false;
    }
    let r1 = grid[r + 1][c] + grid[r + 1][c + 1] + grid[r + 1][c + 2];
    if r1 != 15 {
        return false;
    }
    let r2 = grid[r + 2][c] + grid[r + 2][c + 1] + grid[r + 2][c + 2];
    if r2 != 15 {
        return false;
    }
    let c0 = grid[r][c] + grid[r + 1][c] + grid[r + 2][c];
    if c0 != 15 {
        return false;
    }
    let c1 = grid[r][c + 1] + grid[r + 1][c + 1] + grid[r + 2][c + 1];
    if c1 != 15 {
        return false;
    }
    let c2 = grid[r][c + 2] + grid[r + 1][c + 2] + grid[r + 2][c + 2];
    if c2 != 15 {
        return false;
    }
    let d0 = grid[r][c] + grid[r + 1][c + 1] + grid[r + 2][c + 2];
    if d0 != 15 {
        return false;
    }
    let d1 = grid[r][c + 2] + grid[r + 1][c + 1] + grid[r + 2][c];
    if d1 != 15 {
        return false;
    }
    true
}
// array
#[test]
fn test1_840() {
    assert_eq!(
        num_magic_squares_inside(vec![vec![4, 3, 8, 4], vec![9, 5, 1, 9], vec![2, 7, 6, 2]]),
        1
    );
}
