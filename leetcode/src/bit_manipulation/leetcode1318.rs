// https://leetcode.com/problems/minimum-flips-to-make-a-or-b-equal-to-c/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
    let mut res = 0;
    for i in 0..32 {
        let aa = (a >> i) & 1;
        let bb = (b >> i) & 1;
        let cc = (c >> i) & 1;
        if cc == 0 {
            if aa == 1 {
                res += 1;
            }
            if bb == 1 {
                res += 1;
            }
        } else if aa == 0 && bb == 0 {
            res += 1;
        }
    }
    res
}
// bit_manipulation
#[test]
fn test1_1318() {
    assert_eq!(min_flips(2, 6, 5), 3);
    assert_eq!(min_flips(4, 2, 7), 1);
    assert_eq!(min_flips(1, 2, 3), 0);
}
