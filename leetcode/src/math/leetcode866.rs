// https://leetcode.com/problems/prime-palindrome/
#![allow(clippy::many_single_char_names)]
// Runtime: 36 ms
// Memory Usage: 2.1 MB
pub fn prime_palindrome(n: i32) -> i32 {
    if 8 <= n && n <= 11 {
        return 11;
    }
    for i in 1..100_000 {
        let mut x = format!("{}", i).chars().collect::<Vec<_>>();
        let mut y = x.clone();
        y.reverse();
        x.extend(y.iter().skip(1));
        let s = x.iter().collect::<String>();
        let v = s.parse::<i32>().unwrap();
        if v < n {
            continue;
        }
        if is_prime(v) {
            return v;
        }
    }
    0
}

fn is_prime(x: i32) -> bool {
    if x < 2 || x % 2 == 0 {
        return x == 2;
    }
    let mut i = 3;
    while i * i <= x {
        if x % i == 0 {
            return false;
        } else {
            i += 2;
        }
    }
    true
}
// math
#[test]
fn test1_866() {
    assert_eq!(prime_palindrome(6), 7);
    assert_eq!(prime_palindrome(8), 11);
    assert_eq!(prime_palindrome(13), 101);
}
