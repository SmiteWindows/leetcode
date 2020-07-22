// https://leetcode.com/problems/self-dividing-numbers/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let mut res = vec![];
    for i in left..=right {
        if is_self_dividing(i) {
            res.push(i);
        }
    }
    res
}

fn is_self_dividing(x: i32) -> bool {
    let mut n = x;
    while n > 0 {
        let last = n % 10;
        if last == 0 {
            return false;
        } else {
            if x % last != 0 {
                return false;
            }
            n /= 10;
        }
    }
    true
}
// math
#[test]
fn test1_728() {
    assert_eq!(
        self_dividing_numbers(1, 22),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
    );
}
