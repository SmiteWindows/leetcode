// https://leetcode-cn.com/problems/find-numbers-with-even-number-of-digits/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn find_numbers(nums: Vec<i32>) -> i32 {
    fn digits(mut n: i32) -> i32 {
        let mut res = 0;
        while n != 0 {
            n /= 10;
            res += 1;
        }
        res
    }

    let mut res = 0;
    for n in nums {
        if digits(n) % 2 == 0 {
            res += 1;
        }
    }
    res
}
// array
#[test]
fn test1_1295() {
    assert_eq!(find_numbers(vec![12, 345, 2, 6, 7896]), 2);
    assert_eq!(find_numbers(vec![555, 901, 482, 1771]), 1);
}
