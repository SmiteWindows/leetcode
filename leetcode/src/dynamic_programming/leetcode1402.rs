// https://leetcode-cn.com/problems/reducing-dishes/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
    satisfaction.sort_unstable();
    let n = satisfaction.len();
    let mut total = 0;
    let mut res = 0;
    for s in satisfaction.iter().rev() {
        if s + total < 0 {
            break;
        } else {
            total += s;
            res += total;
        }
    }
    res
}
// dynamic_programming
#[test]
fn test1_1402() {
    assert_eq!(max_satisfaction(vec![-1, -8, 0, 5, -9]), 14);
    assert_eq!(max_satisfaction(vec![4, 3, 2]), 20);
    assert_eq!(max_satisfaction(vec![-1, -4, -5]), 0);
    assert_eq!(max_satisfaction(vec![-2, 5, -1, 0, 3, -3]), 35);
}
