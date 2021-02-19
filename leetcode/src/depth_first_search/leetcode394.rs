// https://leetcode-cn.com/problems/decode-string/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn decode_string(s: String) -> String {
    let s: Vec<char> = s.chars().collect();
    let mut i = 0;
    decode(&s, s.len(), &mut i)
}
fn decode(s: &[char], m: usize, i: &mut usize) -> String {
    let mut res = "".to_string();
    while *i < m && s[*i] != ']' {
        if s[*i].is_digit(10) {
            let mut n = 0;
            while *i < m && s[*i].is_digit(10) {
                n *= 10;
                n += (s[*i] as u8 - b'0') as i32;
                *i += 1;
            }
            *i += 1;
            let t = decode(s, m, i);
            *i += 1;
            for _ in 0..n {
                res += &t;
            }
        } else {
            res.push(s[*i]);
            *i += 1;
        }
    }
    res
}
// stack depth_first_search
#[test]
fn test2_394() {
    assert_eq!(
        decode_string("3[a]2[bc]".to_string()),
        "aaabcbc".to_string()
    );
    assert_eq!(
        decode_string("3[a2[c]]".to_string()),
        "accaccacc".to_string()
    );
    assert_eq!(
        decode_string("2[abc]3[cd]ef".to_string()),
        "abcabccdcdcdef".to_string()
    );
}
