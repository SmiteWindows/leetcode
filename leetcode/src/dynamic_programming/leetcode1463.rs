// https://leetcode-cn.com/problems/cherry-pickup-ii/
#![allow(clippy::many_single_char_names)]
// Runtime: 144 ms
// Memory Usage: 10.3 MB
use std::collections::HashMap;
pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut memo = HashMap::new();
    dp(0, 0, m - 1, &mut memo, &grid, n, m)
}

fn dp(
    i: usize,
    j: usize,
    k: usize,
    memo: &mut HashMap<(usize, usize, usize), i32>,
    grid: &[Vec<i32>],
    n: usize,
    m: usize,
) -> i32 {
    if let Some(&res) = memo.get(&(i, j, k)) {
        return res;
    }

    let mut res = if k != j {
        grid[i][j] + grid[i][k]
    } else {
        grid[i][j]
    };

    if i + 1 < n {
        let mut max = 0;
        if j > 0 && k > 0 {
            max = max.max(dp(i + 1, j - 1, k - 1, memo, grid, n, m));
        }
        if k > 0 {
            max = max.max(dp(i + 1, j, k - 1, memo, grid, n, m));
        }
        if j + 1 < m && k > 0 {
            max = max.max(dp(i + 1, j + 1, k - 1, memo, grid, n, m));
        }
        if j > 0 {
            max = max.max(dp(i + 1, j - 1, k, memo, grid, n, m));
        }
        max = max.max(dp(i + 1, j, k, memo, grid, n, m));
        if j + 1 < m {
            max = max.max(dp(i + 1, j + 1, k, memo, grid, n, m));
        }
        if j > 0 && k + 1 < m {
            max = max.max(dp(i + 1, j - 1, k + 1, memo, grid, n, m));
        }
        if k + 1 < m {
            max = max.max(dp(i + 1, j, k + 1, memo, grid, n, m));
        }
        if j + 1 < m && k + 1 < m {
            max = max.max(dp(i + 1, j + 1, k + 1, memo, grid, n, m));
        }
        res += max;
    }
    memo.insert((i, j, k), res);
    res
}
// dynamic_programming
#[test]
fn test1_1463() {
    use leetcode_prelude::vec2;
    assert_eq!(
        cherry_pickup(vec2![[3, 1, 1], [2, 5, 1], [1, 5, 5], [2, 1, 1]]),
        24
    );
    assert_eq!(
        cherry_pickup(vec2![
            [1, 0, 0, 0, 0, 0, 1],
            [2, 0, 0, 0, 0, 3, 0],
            [2, 0, 9, 0, 0, 0, 0],
            [0, 3, 0, 5, 4, 0, 0],
            [1, 0, 2, 3, 0, 0, 6]
        ]),
        28
    );
    assert_eq!(
        cherry_pickup(vec2![
            [1, 0, 0, 3],
            [0, 0, 0, 3],
            [0, 0, 3, 3],
            [9, 0, 3, 3]
        ]),
        22
    );
    assert_eq!(cherry_pickup(vec2![[1, 1], [1, 1]]), 4);
}
