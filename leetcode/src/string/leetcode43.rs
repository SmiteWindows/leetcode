// https://leetcode-cn.com/problems/multiply-strings/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn multiply(num1: String, num2: String) -> String {
    if num1 == "0" || num2 == "0" {
        return "0".to_string();
    }
    let n1 = num1.len();
    let n2 = num2.len();
    let n3 = n1 + n2;
    let mut res = vec![0; n3];
    for (i, c1) in num1.char_indices().rev() {
        let c1 = c1.to_digit(10).unwrap();
        for (j, c2) in num2.char_indices().rev() {
            let c2 = c2.to_digit(10).unwrap();
            let sum = res[i + j + 1] + c1 * c2;
            res[i + j + 1] = sum % 10;
            res[i + j] += sum / 10;
        }
    }
    while let Some(0) = res.first() {
        res.remove(0);
    }
    res.into_iter().map(|x| (x as u8 + b'0') as char).collect()
}
// math string
#[test]
fn test1_43() {
    assert_eq!(
        multiply("123".to_string(), "45".to_string()),
        "5535".to_string()
    );
    assert_eq!(
        multiply("123".to_string(), "456".to_string()),
        "56088".to_string()
    );
    assert_eq!(multiply("2".to_string(), "3".to_string()), "6".to_string());
    assert_eq!(multiply("0".to_string(), "3".to_string()), "0".to_string());
}
