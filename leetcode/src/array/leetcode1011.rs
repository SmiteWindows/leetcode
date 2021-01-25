// https://leetcode-cn.com/problems/capacity-to-ship-packages-within-d-days/
// Runtime: 8 ms
// Memory Usage: 2.5 MB
pub fn ship_within_days(weights: Vec<i32>, d: i32) -> i32 {
    let mut sum = 0;
    let mut max = 0;
    for &w in &weights {
        sum += w;
        max = max.max(w);
    }
    let mut l = max;
    let mut h = sum;
    while l < h {
        let m = l + (h - l) / 2;
        if days(m, &weights) <= d {
            h = m;
        } else {
            l = m + 1;
        }
    }
    l
}

fn days(capacity: i32, weights: &[i32]) -> i32 {
    let mut cur = 0;
    let mut res = 1;
    for &w in weights {
        cur += w;
        if cur > capacity {
            res += 1;
            cur = w;
        }
    }
    res
}
// binary_search array
#[test]
fn test2_1011() {
    assert_eq!(ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 15);
    assert_eq!(ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
    assert_eq!(ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
}
