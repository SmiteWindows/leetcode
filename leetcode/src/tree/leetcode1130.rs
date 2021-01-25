// https://leetcode-cn.com/problems/minimum-cost-tree-from-leaf-values/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut stack = vec![i32::MAX];
    for x in arr {
        while x >= *stack.last().unwrap() {
            let mid = stack.pop().unwrap();
            let y = *stack.last().unwrap();
            res += mid * x.min(y);
        }
        stack.push(x);
    }
    while stack.len() > 2 {
        let x = stack.pop().unwrap();
        let y = *stack.last().unwrap();
        res += x * y;
    }
    res
}
// tree stack dynamic_programming
#[test]
fn test1_1130() {
    assert_eq!(mct_from_leaf_values(vec![6, 2, 4]), 32);
}
