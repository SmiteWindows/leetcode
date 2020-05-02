// https://leetcode.com/problems/ugly-number-iii/
pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
    todo!()
}
// math binary_search
#[test]
#[ignore]
fn test1_1201() {
    assert_eq!(nth_ugly_number(3, 2, 3, 5), 4);
    assert_eq!(nth_ugly_number(4, 2, 3, 4), 6);
    assert_eq!(nth_ugly_number(5, 2, 11, 13), 10);
    assert_eq!(
        nth_ugly_number(1000000000, 2, 217983653, 336916467),
        1999999984
    );
}
