// https://leetcode.com/problems/base-7/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn convert_to_base7(num: i32) -> String {
    let mut num = num;
    if num == 0 {
        return "0".to_string();
    }
    let mut base_7 = vec![];
    let minus = num < 0;
    if minus {
        num = -num;
    }
    while num > 0 {
        let c = ((num % 7) as u8 + b'0') as char;
        base_7.push(c);
        num /= 7;
    }
    if minus {
        base_7.push('-');
    }
    base_7.reverse();
    base_7.iter().collect()
}
#[test]
fn test504() {
    assert_eq!(convert_to_base7(100), "202".to_string());
    assert_eq!(convert_to_base7(-7), "-10".to_string());
}
