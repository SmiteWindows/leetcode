// https://leetcode.com/problems/happy-number/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
pub fn is_happy(n: i32) -> bool {
    let mut slow = n;
    let mut fast = get_next(n);
    while fast != 1 && slow != fast {
        slow = get_next(slow);
        fast = get_next(get_next(fast));
    }
    fast == 1
}

fn get_next(mut n: i32) -> i32 {
    let mut sum = 0;
    while n > 0 {
        let d = n % 10;
        n /= 10;
        sum += d * d;
    }
    sum
}
// math hash_table
#[test]
fn test1_202() {
    assert_eq!(is_happy(19), true);
}
