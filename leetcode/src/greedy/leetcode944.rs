// https://leetcode-cn.com/problems/delete-columns-to-make-sorted/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
pub fn min_deletion_size(a: Vec<String>) -> i32 {
    let mut d = 0;
    let a = a.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
    let n = a.len();
    let m = a[0].len();
    for i in 0..m {
        for j in 1..n {
            if a[j][i] < a[j - 1][i] {
                d += 1;
                break;
            }
        }
    }
    d
}
// greedy
#[test]
fn test1_944() {
    use leetcode_prelude::vec_string;
    assert_eq!(min_deletion_size(vec_string!["cba", "daf", "ghi"]), 1);
    assert_eq!(min_deletion_size(vec_string!["a", "b"]), 0);
    assert_eq!(min_deletion_size(vec_string!["zyx", "wvu", "tsr"]), 3);
}
