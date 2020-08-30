// https://leetcode-cn.com/problems/array-nesting/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
pub fn array_nesting(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let n = nums.len();
    let mut visited = vec![false; n];
    for i in 0..n {
        if visited[i] {
            continue;
        }
        let mut j = i;
        let mut length = 0;
        while !visited[j] {
            visited[j] = true;
            j = nums[j] as usize;
            length += 1;
        }
        res = res.max(length);
    }
    res
}
// array
#[test]
fn test1_565() {
    assert_eq!(array_nesting(vec![5, 4, 0, 3, 1, 6, 2]), 4);
}
