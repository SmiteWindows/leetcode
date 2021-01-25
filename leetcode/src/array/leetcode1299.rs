// https://leetcode-cn.com/problems/replace-elements-with-greatest-element-on-right-side/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
    let mut max = -1;
    let n = arr.len();
    for val in arr.iter_mut().rev() {
        let t = max;
        max = max.max(*val);
        *val = t;
    }
    arr
}
// array
#[test]
fn test1_1299() {
    assert_eq!(
        replace_elements(vec![17, 18, 5, 4, 6, 1]),
        vec![18, 6, 6, 6, 1, -1]
    );
}
