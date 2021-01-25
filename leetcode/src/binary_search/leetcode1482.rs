// https://leetcode-cn.com/problems/minimum-number-of-days-to-make-m-bouquets/
// Runtime: 28 ms
// Memory Usage: 3.8 MB
pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    let m = m;
    let k = k as usize;
    let n = bloom_day.len();
    if n < m as usize * k {
        return -1;
    }
    let mut left = 1;
    let mut right = i32::MAX;
    while left < right {
        let mid = left + (right - left) / 2;
        let mut group = 0;
        let mut count = 0;
        for &bd in bloom_day.iter() {
            if bd > mid {
                count = 0;
            } else {
                count += 1;
                if count >= k {
                    count = 0;
                    group += 1;
                }
            }
        }
        if group < m {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}
// array binary_search
#[test]
fn test2_1482() {
    assert_eq!(min_days(vec![1, 10, 3, 10, 2], 3, 1), 3);
    assert_eq!(min_days(vec![1, 10, 3, 10, 2], 3, 2), -1);
    assert_eq!(min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3), 12);
    assert_eq!(min_days(vec![1000000000, 1000000000], 1, 1), 1000000000);
    assert_eq!(min_days(vec![1, 10, 2, 9, 3, 8, 4, 7, 5, 6], 4, 2), 9);
}
