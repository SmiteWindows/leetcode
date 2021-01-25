// https://leetcode-cn.com/problems/convert-integer-to-the-sum-of-two-no-zero-integers/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
    let mut v = vec![true; 10_000];
    v[0] = false;
    for i in 0..10 {
        for j in 0..10 {
            for k in 0..10 {
                for l in 0..10 {
                    let x = i * 1000 + j * 100 + k * 10 + l;
                    if i != 0 && (j == 0 || k == 0 || l == 0) {
                        v[x] = false;
                    }
                    if j != 0 && (k == 0 || l == 0) {
                        v[x] = false;
                    }
                    if k != 0 && l == 0 {
                        v[x] = false;
                    }
                }
            }
        }
    }
    for a in 1..n {
        let b = n - a;
        if v[a as usize] && v[b as usize] {
            return vec![a, b];
        }
    }
    vec![]
}
// math
#[test]
fn test1_1317() {
    assert_eq!(get_no_zero_integers(2), vec![1, 1]);
    assert_eq!(get_no_zero_integers(11), vec![2, 9]);
    assert_eq!(get_no_zero_integers(10000), vec![1, 9999]);
    assert_eq!(get_no_zero_integers(69), vec![1, 68]);
    assert_eq!(get_no_zero_integers(1010), vec![11, 999]);
}
