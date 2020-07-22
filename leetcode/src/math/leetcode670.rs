// https://leetcode.com/problems/maximum-swap/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn maximum_swap(num: i32) -> i32 {
    let mut s = num.to_string().chars().collect::<Vec<_>>();
    let n = s.len();
    let mut last = vec![0; 10];
    for i in 0..n {
        last[(s[i] as u8 - b'0') as usize] = i;
    }
    for i in 0..n {
        let d = (s[i] as u8 - b'0') as usize;
        for j in (d + 1..10).rev() {
            if last[j] > i {
                s.swap(i, last[j]);
                return s.into_iter().collect::<String>().parse::<i32>().unwrap();
            }
        }
    }
    num
}
// math array
#[test]
fn test1_670() {
    assert_eq!(maximum_swap(2736), 7236);
    assert_eq!(maximum_swap(9973), 9973);
}
