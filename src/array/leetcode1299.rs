// https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;
    let mut max = -1;
    let n = arr.len();
    for i in (0..n).rev() {
        let t = max;
        max = i32::max(arr[i], max);
        arr[i] = t;
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
