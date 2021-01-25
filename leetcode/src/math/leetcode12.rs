// https://leetcode-cn.com/problems/integer-to-roman/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn int_to_roman(mut num: i32) -> String {
    let mapping = vec![
        (1, "I"),
        (2, "II"),
        (3, "III"),
        (4, "IV"),
        (5, "V"),
        (6, "VI"),
        (7, "VII"),
        (8, "VIII"),
        (9, "IX"),
        (10, "X"),
        (20, "XX"),
        (30, "XXX"),
        (40, "XL"),
        (50, "L"),
        (60, "LX"),
        (70, "LXX"),
        (80, "LXXX"),
        (90, "XC"),
        (100, "C"),
        (200, "CC"),
        (300, "CCC"),
        (400, "CD"),
        (500, "D"),
        (600, "DC"),
        (700, "DCC"),
        (800, "DCCC"),
        (900, "CM"),
        (1000, "M"),
        (2000, "MM"),
        (3000, "MMM"),
    ];
    let mut res = "".to_string();
    for &(x, s) in mapping.iter().rev() {
        if num >= x {
            res += s;
            num -= x;
        } else {
            continue;
        }
        if num == 0 {
            break;
        }
    }
    res
}
// math string
#[test]
fn test2_12() {
    assert_eq!(int_to_roman(3), "III".to_string());
    assert_eq!(int_to_roman(4), "IV".to_string());
    assert_eq!(int_to_roman(9), "IX".to_string());
    assert_eq!(int_to_roman(58), "LVIII".to_string());
    assert_eq!(int_to_roman(1994), "MCMXCIV".to_string());
}
