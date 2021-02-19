// https://leetcode-cn.com/problems/minimum-domino-rotations-for-equal-row/
#[allow(clippy::many_single_char_names)]
// Runtime: 28 ms
// Memory Usage: 2.3 MB
pub fn min_domino_rotations(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut count = vec![vec![0_usize; 3]; 6];
    let n = a.len();
    for i in 0..n {
        let x = (a[i] - 1) as usize;
        let y = (b[i] - 1) as usize;
        count[x][0] += 1;
        count[y][1] += 1;
        if x == y {
            count[x][2] += 1;
        }
    }
    for ci in count.iter().take(6) {
        if ci[0] + ci[1] - ci[2] >= n {
            return (ci[0].min(ci[1]) - ci[2]) as i32;
        }
    }
    -1
}
// greedy array
#[test]
fn test1_1007() {
    assert_eq!(
        min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2]),
        2
    );
    assert_eq!(
        min_domino_rotations(vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4]),
        -1
    );
}
