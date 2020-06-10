// https://leetcode.com/problems/shifting-letters/
// Runtime: 4 ms
// Memory Usage: 2.6 MB
pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
    let n = s.len();
    let mut s = s.chars().collect::<Vec<_>>();
    let mut prev = 0;
    for i in (0..n).rev() {
        prev += (shifts[i] % 26) as u8;
        prev %= 26;
        s[i] = (b'a' + (s[i] as u8 - b'a' + prev) % 26) as char;
    }
    s.into_iter().collect()
}
// string
#[test]
fn test1_848() {
    assert_eq!(
        shifting_letters(String::from("abc"), vec![3, 5, 9]),
        String::from("rpl")
    );
}
