// https://leetcode-cn.com/problems/minimum-increment-to-make-array-unique/
// Runtime: 12 ms
// Memory Usage: 2.4 MB
pub fn min_increment_for_unique(mut a: Vec<i32>) -> i32 {
    let mut res = 0;
    a.sort_unstable();
    let mut prev = None;
    for x in a {
        if let Some(y) = prev {
            if x <= y {
                res += y + 1 - x;
                prev = Some(y + 1);
            } else {
                prev = Some(x);
            }
        } else {
            prev = Some(x);
        }
    }
    res
}
// array
#[test]
fn test1_945() {
    assert_eq!(min_increment_for_unique(vec![1, 2, 2]), 1);
    assert_eq!(min_increment_for_unique(vec![3, 2, 1, 2, 1, 7]), 6);
}
