// https://leetcode-cn.com/problems/play-with-chips/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn min_cost_to_move_chips(chips: Vec<i32>) -> i32 {
    let mut a = 0;
    let mut b = 0;
    for val in chips {
        if val % 2 == 0 {
            a += 1;
        } else {
            b += 1;
        }
    }
    a.min(b)
}
// math array greedy
#[test]
fn test2_1217() {
    assert_eq!(min_cost_to_move_chips(vec![1, 2, 3]), 1);
    assert_eq!(min_cost_to_move_chips(vec![2, 2, 2, 3, 3]), 2);
}
