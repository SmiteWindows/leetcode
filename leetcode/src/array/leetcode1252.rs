// https://leetcode.com/problems/cells-with-odd-values-in-a-matrix/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
pub fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let m = m as usize;
    let mut arr = vec![vec![0; m]; n];
    let mut res = 0;
    for p in indices {
        let r = p[0] as usize;
        let c = p[1] as usize;
        for j in 0..m {
            arr[r][j] += 1;
            if arr[r][j] % 2 == 1 {
                res += 1;
            } else {
                res -= 1;
            }
        }
        for row in arr.iter_mut().take(n) {
            row[c] += 1;
            if row[c] % 2 == 1 {
                res += 1;
            } else {
                res -= 1;
            }
        }
    }
    res
}
// array
#[test]
fn test1_1252() {
    assert_eq!(odd_cells(2, 3, vec![vec![0, 1], vec![1, 1]]), 6);
    assert_eq!(odd_cells(2, 2, vec![vec![1, 1], vec![0, 0]]), 0);
}
