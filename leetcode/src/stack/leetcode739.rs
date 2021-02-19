// https://leetcode-cn.com/problems/daily-temperatures/
// Runtime: 20 ms
// Memory Usage: 2.5 MB
pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut stack = Vec::new();
    let n = t.len();
    for i in (0..n).rev() {
        while let Some(j) = stack.pop() {
            if t[j] > t[i] {
                stack.push(j);
                break;
            }
        }
        if let Some(j) = stack.last() {
            res.push((j - i) as i32);
        } else {
            res.push(0);
        }
        stack.push(i)
    }
    res.into_iter().rev().collect()
}
// stack hash_table
#[test]
fn test1_739() {
    assert_eq!(
        daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
        vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
}
