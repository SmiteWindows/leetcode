// https://leetcode-cn.com/problems/delete-columns-to-make-sorted-iii/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn min_deletion_size(a: Vec<String>) -> i32 {
    let n = a.len();
    let a: Vec<Vec<char>> = a.into_iter().map(|s| s.chars().collect()).collect();
    let m = a[0].len();
    let mut dp = vec![1; m];
    for i in 1..m {
        'outer: for j in 0..i {
            for ak in a.iter().take(n) {
                if ak[j] > ak[i] {
                    continue 'outer;
                }
            }
            dp[i] = dp[i].max(dp[j] + 1);
        }
    }
    (m - dp.into_iter().max().unwrap()) as i32
}
// dynamic_programming
#[test]
fn test1_960() {
    assert_eq!(
        min_deletion_size(vec![String::from("babca"), String::from("bbazb")]),
        3
    );
    assert_eq!(min_deletion_size(vec![String::from("edcba")]), 4);
    assert_eq!(
        min_deletion_size(vec![
            String::from("ghi"),
            String::from("def"),
            String::from("abc")
        ]),
        0
    );
}
