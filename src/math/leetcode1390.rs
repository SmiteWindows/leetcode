// https://leetcode.com/problems/four-divisors/
// Runtime: 16 ms
// Memory Usage: 2.1 MB
pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    'outer: for x in nums {
        let mut i = 2;
        let mut v = vec![];
        while i * i <= x {
            if x % i == 0 {
                v.push(i);
                if v.len() > 1 {
                    continue 'outer;
                }
            }
            i += 1;
        }
        if v.len() == 1 && v[0] * v[0] != x {
            res += 1 + v[0] + x / v[0] + x;
        }
    }
    res
}
// math
#[test]
fn test1_1390() {
    assert_eq!(sum_four_divisors(vec![21, 4, 7]), 32);
}
