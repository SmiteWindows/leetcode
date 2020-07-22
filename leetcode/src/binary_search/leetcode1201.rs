// https://leetcode.com/problems/ugly-number-iii/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
    let mut left = 0;
    let mut right = 2_000_000_000;
    while left < right {
        let mid = left + (right - left) / 2;
        if count(mid, a as u64, b as u64, c as u64) < n as u64 {
            left = mid + 1;
        } else {
            right = mid
        }
    }
    left as i32
}

fn count(num: u64, a: u64, b: u64, c: u64) -> u64 {
    num / a + num / b + num / c - num / lcm(a, b) - num / lcm(b, c) - num / lcm(a, c)
        + num / lcm(a, lcm(b, c))
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}
// math binary_search
#[test]
fn test2_1201() {
    assert_eq!(nth_ugly_number(3, 2, 3, 5), 4);
    assert_eq!(nth_ugly_number(4, 2, 3, 4), 6);
    assert_eq!(nth_ugly_number(5, 2, 11, 13), 10);
    assert_eq!(
        nth_ugly_number(1000000000, 2, 217983653, 336916467),
        1999999984
    );
}
