// https://leetcode-cn.com/problems/advantage-shuffle/
// Runtime: 24 ms
// Memory Usage: 2.5 MB
type Pair = (usize, i32);
pub fn advantage_count(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let n = a.len();
    let mut b = b.into_iter().enumerate().collect::<Vec<Pair>>();
    let mut l = 0;
    let mut r = n - 1;
    a.sort_unstable();
    b.sort_unstable_by_key(|p| p.1);
    let mut res = vec![0; n];
    for &ai in a.iter().take(n) {
        if ai <= b[l].1 {
            res[b[r].0] = ai;
            r -= 1;
        } else {
            res[b[l].0] = ai;
            l += 1;
        }
    }
    res
}
// greedy array
#[test]
fn test2_870() {
    assert_eq!(
        advantage_count(vec![2, 7, 11, 15], vec![1, 10, 4, 11]),
        vec![2, 11, 7, 15]
    );
    assert_eq!(
        advantage_count(vec![12, 24, 8, 32], vec![13, 25, 32, 11]),
        vec![24, 32, 8, 12]
    );
}
