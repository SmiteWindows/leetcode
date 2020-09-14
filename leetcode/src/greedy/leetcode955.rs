// https://leetcode-cn.com/problems/delete-columns-to-make-sorted-ii/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn min_deletion_size(a: Vec<String>) -> i32 {
    let n = a.len();
    let mut sorted = vec!["".to_string(); n];
    let a = a
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let m = a[0].len();
    for j in 0..m {
        for i in 0..n {
            sorted[i].push(a[i][j]);
        }
        if sorted.windows(2).any(|w| w[0] > w[1]) {
            for si in sorted.iter_mut().take(n) {
                si.pop();
            }
        }
    }
    (m - sorted[0].len()) as i32
}
// greedy
#[test]
fn test1_955() {
    use leetcode_prelude::vec_string;
    assert_eq!(min_deletion_size(vec_string!["ca", "bb", "ac"]), 1);
    assert_eq!(min_deletion_size(vec_string!["xc", "yb", "za"]), 0);
    assert_eq!(min_deletion_size(vec_string!["zyx", "wvu", "tsr"]), 3);
}
