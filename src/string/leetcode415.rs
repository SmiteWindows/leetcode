// https://leetcode.com/problems/add-strings/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn add_strings(num1: String, num2: String) -> String {
    let s1 = num1
        .bytes()
        .map(|x| (x - b'0') as i32)
        .rev()
        .collect::<Vec<_>>();
    let s2 = num2
        .bytes()
        .map(|x| (x - b'0') as i32)
        .rev()
        .collect::<Vec<_>>();
    let mut carry = 0;
    let mut i = 0;
    let mut s3 = vec![];
    while i < s1.len() || i < s2.len() || carry > 0 {
        let mut v = 0;
        if i < s1.len() {
            v += s1[i];
        }
        if i < s2.len() {
            v += s2[i];
        }
        v += carry;
        carry = v / 10;
        s3.push(((v % 10) as u8 + b'0') as char);
        i += 1;
    }
    s3.iter().rev().collect()
}
// string
#[test]
fn test1_415() {
    assert_eq!(
        add_strings(String::from("1"), String::from("2")),
        String::from("3")
    );
    assert_eq!(
        add_strings(String::from("12"), String::from("34")),
        String::from("46")
    );
}
