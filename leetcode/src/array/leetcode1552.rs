// https://leetcode.com/problems/magnetic-force-between-two-balls/
// Runtime: 48 ms
// Memory Usage: 3.8 MB
pub fn max_distance(position: Vec<i32>, m: i32) -> i32 {
    let mut position = position;
    position.sort_unstable();
    let n = position.len();
    let mut low = i32::MAX;
    for w in position.windows(2) {
        low = low.min(w[1] - w[0]);
    }
    let mut high = (position[n - 1] - position[0]) / (m - 1) as i32;
    while low < high {
        let mid = (low + high + 1) / 2;
        let mut prev = position[0];
        let mut count = 1;
        for &pi in position.iter().take(n).skip(1) {
            if pi - prev >= mid {
                count += 1;
                prev = pi;
            }
        }
        if count < m {
            high = mid - 1;
        } else {
            low = mid;
        }
    }
    low
}
// array binary_search
#[test]
fn test1_1552() {
    assert_eq!(max_distance(vec![1, 2, 3, 4, 7], 3), 3);
    assert_eq!(max_distance(vec![5, 4, 3, 2, 1, 1000000000], 2), 999999999);
}
