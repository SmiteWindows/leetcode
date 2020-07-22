// https://leetcode.com/problems/minimum-swaps-to-make-sequences-increasing/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn min_swap(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let n = a.len();
    let mut keep = vec![n; n];
    let mut swap = vec![n; n];
    keep[0] = 0;
    swap[0] = 1;
    for i in 1..n {
        if a[i - 1] < a[i] && b[i - 1] < b[i] {
            keep[i] = keep[i - 1];
            swap[i] = swap[i - 1] + 1;
        }
        if a[i - 1] < b[i] && b[i - 1] < a[i] {
            keep[i] = keep[i].min(swap[i - 1]);
            swap[i] = swap[i].min(keep[i - 1] + 1);
        }
    }
    swap[n - 1].min(keep[n - 1]) as i32
}
// dynamic_programming
#[test]
fn test1_801() {
    assert_eq!(min_swap(vec![1, 3, 5, 4], vec![1, 2, 3, 7]), 1);
}
