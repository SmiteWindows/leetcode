// https://leetcode-cn.com/problems/most-visited-sector-in-a-circular-track/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
    let m = rounds.len();
    let start = rounds[0];
    let end = rounds[m - 1];
    if start == end {
        return vec![start];
    }
    if start < end {
        return (start..=end).collect();
    }
    let mut res: Vec<i32> = (start..=end + n).map(|x| ((x - 1) % n) + 1).collect();
    res.sort_unstable();
    res
}
// array
#[test]
fn test1_1560() {
    assert_eq!(most_visited(4, vec![1, 3, 1, 2]), vec![1, 2]);
    assert_eq!(most_visited(2, vec![2, 1, 2, 1, 2, 1, 2, 1, 2]), vec![2]);
    assert_eq!(most_visited(7, vec![1, 3, 5, 7]), vec![1, 2, 3, 4, 5, 6, 7]);
}
