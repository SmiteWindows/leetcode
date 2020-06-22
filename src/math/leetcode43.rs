// https://leetcode.com/problems/multiply-strings/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn multiply(num1: String, num2: String) -> String {
    let n1 = num1.len();
    let n2 = num2.len();
    let n3 = n1 + n2;
    let mut v1 = vec![0; n1];
    let mut v2 = vec![0; n2];
    let mut v3 = vec![0; n3];
    let mut v4 = vec![];
    for (i, c) in num1.char_indices() {
        v1[n1 - 1 - i] = (c as u8 - b'0') as i32;
    }
    for (i, c) in num2.char_indices() {
        v2[n2 - 1 - i] = (c as u8 - b'0') as i32;
    }
    for i in 0..n1 {
        for j in 0..n2 {
            v3[i + j] += v1[i] * v2[j];
        }
    }
    let mut carry = 0;
    for i in 0..n3 {
        v4.push((v3[i] + carry) % 10);
        carry = (v3[i] + carry) / 10;
    }
    if carry != 0 {
        v4.push(carry);
    }
    while let Some(0) = v4.last() {
        v4.pop();
    }
    v4.reverse();
    if v4.is_empty() {
        "0".to_string()
    } else {
        v4.into_iter().map(|x| (x as u8 + b'0') as char).collect()
    }
}
// math string
#[test]
fn test2_43() {
    assert_eq!(
        multiply(String::from("2"), String::from("3")),
        String::from("6")
    );
    assert_eq!(
        multiply(String::from("123"), String::from("456")),
        String::from("56088")
    );
}
