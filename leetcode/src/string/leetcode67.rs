// https://leetcode-cn.com/problems/add-binary/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn add_binary(a: String, b: String) -> String {
    let mut res = vec![];
    let v_a = a.chars().collect::<Vec<_>>();
    let v_b = b.chars().collect::<Vec<_>>();
    let mut carry = 0;
    let mut idx_a = a.len();
    let mut idx_b = b.len();
    while idx_a > 0 || idx_b > 0 || carry == 1 {
        if idx_a > 0 {
            idx_a -= 1;
            carry += v_a[idx_a] as u8 - 48;
        }
        if idx_b > 0 {
            idx_b -= 1;
            carry += v_b[idx_b] as u8 - 48;
        }
        res.push((carry % 2 + 48) as char);
        carry /= 2;
    }
    res.iter().rev().collect()
}
// math string
#[test]
fn test1_67() {
    assert_eq!(
        add_binary(String::from("11"), String::from("1")),
        String::from("100")
    );
    assert_eq!(
        add_binary(String::from("1010"), String::from("1011")),
        String::from("10101")
    );
}
