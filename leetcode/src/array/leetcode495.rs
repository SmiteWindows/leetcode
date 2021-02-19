// https://leetcode-cn.com/problems/teemo-attacking/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
    let n = time_series.len();
    if n == 0 {
        return 0;
    }
    let mut start = time_series[0];
    let mut res = 0;
    for &time_serie in time_series.iter().take(n).skip(1) {
        let end = time_serie;
        if start + duration > end {
            res += end - start;
        } else {
            res += duration;
        }
        start = end;
    }
    res += duration;
    res
}
// array
#[test]
fn test1_495() {
    assert_eq!(find_poisoned_duration(vec![1, 4], 2), 4);
    assert_eq!(find_poisoned_duration(vec![1, 2], 2), 3);
}
