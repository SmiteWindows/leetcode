// https://leetcode-cn.com/problems/count-number-of-teams/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn num_teams(rating: Vec<i32>) -> i32 {
    let n = rating.len();
    let mut res = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if rating[i] < rating[j] && rating[j] < rating[k] {
                    res += 1;
                }
                if rating[i] > rating[j] && rating[j] > rating[k] {
                    res += 1;
                }
            }
        }
    }
    res
}
// array
#[test]
fn test1_1395() {
    assert_eq!(num_teams(vec![2, 5, 3, 4, 1]), 3);
    assert_eq!(num_teams(vec![2, 1, 3]), 0);
    assert_eq!(num_teams(vec![1, 2, 3, 4]), 4);
}
