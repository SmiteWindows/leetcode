// https://leetcode.com/problems/bag-of-tokens/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn bag_of_tokens_score(tokens: Vec<i32>, p: i32) -> i32 {
    let mut tokens = tokens;
    let mut p = p;
    let n = tokens.len();
    if n == 0 {
        return 0;
    }
    tokens.sort_unstable();
    let mut l = 0;
    let mut r = n - 1;
    let mut point = 0;
    let mut res = 0;
    while l <= r {
        if tokens[l] <= p {
            p -= tokens[l];
            point += 1;
            res = res.max(point);
            l += 1;
        } else if point > 0 {
            p += tokens[r];
            point -= 1;
            r -= 1;
        } else {
            break;
        }
    }
    res
}
// greedy
#[test]
fn test1_948() {
    assert_eq!(bag_of_tokens_score(vec![100], 50), 0);
    assert_eq!(bag_of_tokens_score(vec![100, 200], 150), 1);
    assert_eq!(bag_of_tokens_score(vec![100, 200, 300, 400], 200), 2);
}
