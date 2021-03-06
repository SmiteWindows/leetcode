// https://leetcode-cn.com/problems/find-the-closest-palindrome/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn nearest_palindromic(n: String) -> String {
    let size = n.len();
    let half = size / 2;
    let value = n.parse::<i64>().unwrap();
    let a = 10i64.pow(size as u32);
    let b = 10i64.pow(size as u32 - 1);
    let mut candidates = vec![a - 1, a + 1, b - 1, b + 1];
    if size % 2 == 0 {
        let left = n[..half].parse::<i64>().unwrap();
        candidates.push(combine(left - 1, false));
        candidates.push(combine(left, false));
        candidates.push(combine(left + 1, false));
    } else {
        let left = n[..half + 1].parse::<i64>().unwrap();
        candidates.push(combine(left - 1, true));
        candidates.push(combine(left, true));
        candidates.push(combine(left + 1, true));
    }
    let mut res = (std::i64::MAX, 0);
    for x in candidates {
        let diff = (value - x).abs();
        if diff == 0 {
            continue;
        }
        if (diff, x) < res {
            res = (diff, x);
        }
    }
    (res.1).to_string()
}

fn combine(left: i64, odd: bool) -> i64 {
    let l = left.to_string();
    l.chars()
        .chain(l.chars().rev().skip(if odd { 1 } else { 0 }))
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
}
// string
#[test]
fn test1_564() {
    assert_eq!(nearest_palindromic("123".to_string()), "121".to_string());
}
