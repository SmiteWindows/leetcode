// https://leetcode.com/problems/champagne-tower/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    let query_row = query_row as usize;
    let query_glass = query_glass as usize;
    let mut a = vec![vec![0.0; 101]; 101];
    a[0][0] = poured as f64;
    for i in 0..99 {
        for j in 0..=i {
            if a[i][j] > 1.0 {
                let overflow = a[i][j] - 1.0;
                a[i + 1][j] += 0.5 * overflow;
                a[i + 1][j + 1] += 0.5 * overflow;
                a[i][j] = 1.0;
            }
        }
    }
    a[query_row as usize][query_glass as usize]
}
#[test]
fn test799() {
    use leetcode_prelude::assert_approx_eq;
    assert_approx_eq!(champagne_tower(1, 1, 1), 0.0);
    assert_approx_eq!(champagne_tower(2, 1, 1), 0.5);
}
