// https://leetcode-cn.com/problems/decrypt-string-from-alphabet-to-integer-mapping/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn freq_alphabets(s: String) -> String {
    let mut v = vec![];
    let mut u = s.chars().collect::<Vec<_>>();
    while let Some(c) = u.pop() {
        let d = match c {
            '#' => (u.pop().unwrap() as u8 - b'0') + 10 * (u.pop().unwrap() as u8 - b'0'),
            _ => c as u8 - b'0',
        } - 1;
        v.insert(0, (b'a' + d) as char);
    }
    v.iter().collect()
}
// string
#[test]
fn test1_1309() {
    assert_eq!(freq_alphabets("10#11#12".to_string()), "jkab".to_string());
    assert_eq!(freq_alphabets("1326#".to_string()), "acz".to_string());
    assert_eq!(freq_alphabets("25#".to_string()), "y".to_string());
    assert_eq!(
        freq_alphabets("12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#".to_string()),
        "abcdefghijklmnopqrstuvwxyz".to_string()
    );
}
