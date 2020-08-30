// https://leetcode-cn.com/problems/max-difference-you-can-get-from-changing-an-integer/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn max_diff(num: i32) -> i32 {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for i in 0..10 {
        for j in 0..10 {
            let mut s = num.to_string().bytes().collect::<Vec<_>>();
            for k in s.iter_mut() {
                if *k == i + b'0' {
                    *k = j + b'0';
                }
            }
            if s[0] == b'0' {
                continue;
            }
            let x = s
                .into_iter()
                .map(|b| b as char)
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            if x == 0 {
                continue;
            }
            min = min.min(x);
            max = max.max(x);
        }
    }
    max - min
}
// string
#[test]
fn test1_1432() {
    assert_eq!(max_diff(555), 888);
    assert_eq!(max_diff(9), 8);
    assert_eq!(max_diff(123456), 820000);
    assert_eq!(max_diff(10000), 80000);
    assert_eq!(max_diff(9288), 8700);
}
