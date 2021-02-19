// https://leetcode-cn.com/problems/maximize-distance-to-closest-person/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
    let mut first = None;
    let mut last = None;
    let mut prev = None;
    let n = seats.len();
    let mut max = 0;
    for (i, &seat) in seats.iter().enumerate().take(n) {
        if seat == 1 {
            if first.is_none() {
                first = Some(i);
            }
            if let Some(j) = prev {
                max = max.max((i - j) / 2);
            }
            prev = Some(i);
            last = Some(i);
        }
    }
    max = max.max(first.unwrap());
    max = max.max(n - 1 - last.unwrap());
    max as i32
}
// array
#[test]
fn test1_849() {
    assert_eq!(max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1]), 2);
    assert_eq!(max_dist_to_closest(vec![1, 0, 0, 0]), 3);
}
