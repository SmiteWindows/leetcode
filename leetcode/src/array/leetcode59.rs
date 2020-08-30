// https://leetcode-cn.com/problems/spiral-matrix-ii/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use Direction::{Down, Left, Right, Up};
pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut res = vec![vec![0; n]; n];
    let mut i = 0;
    let mut j = 0;
    let mut d = Right;
    for k in 1..=n * n {
        res[i][j] = k as i32;
        match d {
            Right => {
                if j + 1 < n && res[i][j + 1] == 0 {
                    j += 1;
                } else {
                    d = Down;
                    i += 1;
                }
            }
            Down => {
                if i + 1 < n && res[i + 1][j] == 0 {
                    i += 1;
                } else {
                    d = Left;
                    j -= 1;
                }
            }
            Left => {
                if j > 0 && res[i][j - 1] == 0 {
                    j -= 1;
                } else {
                    d = Up;
                    i -= 1;
                }
            }
            Up => {
                if i > 0 && res[i - 1][j] == 0 {
                    i -= 1;
                } else {
                    d = Right;
                    j += 1;
                }
            }
        }
    }
    res
}

enum Direction {
    Right,
    Down,
    Left,
    Up,
}
// array
#[test]
fn test1_59() {
    assert_eq!(
        generate_matrix(3),
        vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
    );
}
