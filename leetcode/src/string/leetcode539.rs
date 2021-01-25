// https://leetcode-cn.com/problems/minimum-time-difference/
// Runtime: 0 ms
// Memory Usage: 3.1 MB
pub fn find_min_difference(time_points: Vec<String>) -> i32 {
    let n = time_points.len();
    const M: usize = 1440_usize;
    if n > M {
        return 0;
    }
    let mut a = [false; M];
    for time_point in time_points {
        let t = clock_to_minute(time_point);
        if a[t as usize] {
            return 0;
        } else {
            a[t as usize] = true;
        }
    }
    let mut first = None;
    let mut last = None;
    let mut prev = None;
    let mut min = M;
    for (i, &ai) in a.iter().enumerate().take(M) {
        if ai {
            if first == None {
                prev = Some(i);
                first = Some(i);
            } else {
                min = min.min(i - prev.unwrap());
                prev = Some(i);
                last = Some(i);
            }
        }
    }
    min = min.min(first.unwrap() + M - last.unwrap() as usize);
    min as i32
}

fn clock_to_minute(s: String) -> usize {
    let parts = s
        .split(':')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    parts[0] * 60 + parts[1]
}
// string
#[test]
fn test1_539() {
    use leetcode_prelude::vec_string;
    assert_eq!(find_min_difference(vec_string!["23:59", "00:00"]), 1);
}
