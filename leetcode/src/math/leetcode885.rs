// https://leetcode-cn.com/problems/spiral-matrix-iii/
// Runtime: 8 ms
// Memory Usage: 2.3 MB
pub fn spiral_matrix_iii(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let total = (r * c) as usize;
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut d = 0;
    let mut step = 1;
    let mut i = r0;
    let mut j = c0;
    let mut k = 0;
    while res.len() < total {
        if i >= 0 && i < r && j >= 0 && j < c {
            res.push(vec![i, j]);
        }
        i += directions[d].0;
        j += directions[d].1;
        k += 1;
        if k == step {
            d += 1;
            d %= 4;
            k = 0;
            if d % 2 == 0 {
                step += 1;
            }
        }
    }
    res
}
// math
#[test]
fn test1_885() {
    use leetcode_prelude::vec2;
    assert_eq!(
        spiral_matrix_iii(1, 4, 0, 0),
        vec2![[0, 0], [0, 1], [0, 2], [0, 3]]
    );
    assert_eq!(
        spiral_matrix_iii(5, 6, 1, 4),
        vec2![
            [1, 4],
            [1, 5],
            [2, 5],
            [2, 4],
            [2, 3],
            [1, 3],
            [0, 3],
            [0, 4],
            [0, 5],
            [3, 5],
            [3, 4],
            [3, 3],
            [3, 2],
            [2, 2],
            [1, 2],
            [0, 2],
            [4, 5],
            [4, 4],
            [4, 3],
            [4, 2],
            [4, 1],
            [3, 1],
            [2, 1],
            [1, 1],
            [0, 1],
            [4, 0],
            [3, 0],
            [2, 0],
            [1, 0],
            [0, 0]
        ]
    );
}
