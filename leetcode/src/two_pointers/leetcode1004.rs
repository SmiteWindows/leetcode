// https://leetcode-cn.com/problems/max-consecutive-ones-iii/
// Runtime: 8 ms
// Memory Usage: 2.2 MB
pub fn longest_ones(a: Vec<i32>, k: i32) -> i32 {
    let n = a.len();
    let mut sum = 0;
    let mut res = 0;
    let mut start = 0;
    let mut end = 0;
    while end < n {
        if sum <= k {
            if a[end] == 0 {
                sum += 1;
            }
            end += 1;
        } else {
            if a[start] == 0 {
                sum -= 1;
            }
            start += 1;
        }
        if sum <= k {
            res = res.max(end - start);
        }
    }
    res as i32
}
// sliding_window two_pointers
#[test]
fn test2_1004() {
    assert_eq!(longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2), 6);
    assert_eq!(
        longest_ones(
            vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
            3
        ),
        10
    );
}
