// https://leetcode-cn.com/problems/largest-plus-sign/
// Runtime: 60 ms
// Memory Usage: 11.1 MB
pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut grid = vec![vec![1; n]; n];
    let mut left = vec![vec![0; n]; n];
    let mut top = vec![vec![0; n]; n];
    let mut right = vec![vec![0; n]; n];
    let mut bottom = vec![vec![0; n]; n];
    for mine in mines {
        let i = mine[0] as usize;
        let j = mine[1] as usize;
        grid[i][j] = 0;
    }
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 {
                if j > 0 {
                    left[i][j] = left[i][j - 1] + 1;
                } else {
                    left[i][j] = 1;
                }
            }
        }
    }
    for j in 0..n {
        for i in 0..n {
            if grid[i][j] == 1 {
                if i > 0 {
                    top[i][j] = top[i - 1][j] + 1;
                } else {
                    top[i][j] = 1;
                }
            }
        }
    }
    for i in 0..n {
        for j in (0..n).rev() {
            if grid[i][j] == 1 {
                if j + 1 < n {
                    right[i][j] = right[i][j + 1] + 1;
                } else {
                    right[i][j] = 1;
                }
            }
        }
    }
    for j in 0..n {
        for i in (0..n).rev() {
            if grid[i][j] == 1 {
                if i + 1 < n {
                    bottom[i][j] = bottom[i + 1][j] + 1;
                } else {
                    bottom[i][j] = 1;
                }
            }
        }
    }
    let mut res = 0;
    for i in 0..n {
        for j in 0..n {
            let mut min = n;
            min = min.min(left[i][j]);
            min = min.min(right[i][j]);
            min = min.min(top[i][j]);
            min = min.min(bottom[i][j]);
            res = res.max(min);
        }
    }
    res as i32
}
// dynamic_programming
#[test]
fn test1_764() {
    use leetcode_prelude::vec2;
    assert_eq!(order_of_largest_plus_sign(5, vec2![[4, 2]]), 2);
    assert_eq!(order_of_largest_plus_sign(2, vec![]), 1);
    assert_eq!(order_of_largest_plus_sign(1, vec2![[0, 0]]), 0);
}
