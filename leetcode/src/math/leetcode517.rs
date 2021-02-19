// https://leetcode-cn.com/problems/super-washing-machines/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn find_min_moves(machines: Vec<i32>) -> i32 {
    let n = machines.len();
    let sum: i32 = machines.iter().sum();
    if sum % n as i32 != 0 {
        return -1;
    }
    let avg = sum / n as i32;
    let mut res = 0;
    let mut count = 0;
    for i in (1..n).rev() {
        let diff = machines[i] - avg;
        res = res.max(diff);
        count += diff;
        res = res.max(count.abs());
    }
    res
}
// math dynamic_programming
#[test]
fn test1_517() {
    assert_eq!(find_min_moves(vec![1, 0, 5]), 3);
    assert_eq!(find_min_moves(vec![0, 3, 0]), 2);
    assert_eq!(find_min_moves(vec![0, 2, 0]), -1);
}
