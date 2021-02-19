// https://leetcode-cn.com/problems/max-chunks-to-make-sorted/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut res = 0;
    for (i, &ai) in arr.iter().enumerate() {
        max = max.max(ai);
        if max == i as i32 {
            res += 1;
        }
    }
    res
}
// array
#[test]
fn test1_769() {
    assert_eq!(max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
    assert_eq!(max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
}
