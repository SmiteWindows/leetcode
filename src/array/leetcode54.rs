// https://leetcode.com/problems/spiral-matrix/
// Runtime: 0 ms
// Memory Usage: 2 MB
use Direction::{Down, Left, Right, Up};
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let n = matrix.len();
    if n == 0 {
        return vec![];
    }
    let m = matrix[0].len();
    if m == 0 {
        return vec![];
    }
    let mut res = vec![];
    let mut i = 0;
    let mut j = 0;
    let mut left = 0;
    let mut top = 0;
    let mut bottom = n - 1;
    let mut right = m - 1;
    let mut direction = Right;
    loop {
        res.push(matrix[i][j]);
        match direction {
            Right => {
                if j < right {
                    j += 1;
                } else {
                    if top < bottom {
                        top += 1;
                        direction = Down;
                        i += 1;
                    } else {
                        break;
                    }
                }
            }
            Down => {
                if i < bottom {
                    i += 1;
                } else {
                    if left < right {
                        right -= 1;
                        direction = Left;
                        j -= 1;
                    } else {
                        break;
                    }
                }
            }
            Left => {
                if j > left {
                    j -= 1;
                } else {
                    if top < bottom {
                        bottom -= 1;
                        direction = Up;
                        i -= 1;
                    } else {
                        break;
                    }
                }
            }
            Up => {
                if i > top {
                    i -= 1;
                } else {
                    if left < right {
                        left += 1;
                        direction = Right;
                        j += 1;
                    } else {
                        break;
                    }
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
fn test1_54() {
    assert_eq!(
        spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
    );
    assert_eq!(
        spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12]
        ]),
        vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
    );
}
