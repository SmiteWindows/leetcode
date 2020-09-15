// https://leetcode-cn.com/problems/find-the-minimum-number-of-fibonacci-numbers-whose-sum-is-k/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn find_min_fibonacci_numbers(mut k: i32) -> i32 {
    let mut v = vec![1, 1];
    let mut i = 1;
    while v[i] + v[i - 1] <= k {
        v.push(v[i] + v[i - 1]);
        i += 1;
    }
    let mut count = 0;
    for x in v.into_iter().rev() {
        if x <= k {
            k -= x;
            count += 1;
        }
    }
    count
}
// array greedy
#[test]
fn test1_1414() {
    assert_eq!(find_min_fibonacci_numbers(7), 2);
    assert_eq!(find_min_fibonacci_numbers(10), 2);
    assert_eq!(find_min_fibonacci_numbers(19), 3);
}
