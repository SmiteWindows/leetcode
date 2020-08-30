// https://leetcode-cn.com/problems/decoded-string-at-index/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn decode_at_index(s: String, k: i32) -> String {
    let mut k = k as u64;
    let s = s.chars().collect::<Vec<_>>();
    let mut size = 0;
    for &i in s.iter() {
        match i {
            '0'..='9' => {
                size *= (i as u8 - b'0') as u64;
            }
            _ => {
                size += 1;
            }
        }
    }
    let mut res = "".to_string();
    for &ri in s.iter().rev() {
        k %= size;
        match ri {
            '0'..='9' => {
                size /= (ri as u8 - b'0') as u64;
            }
            _ => {
                if k == 0 {
                    res = ri.to_string();
                    break;
                } else {
                    size -= 1;
                }
            }
        }
    }
    res
}
// stack
#[test]
fn test1_880() {
    assert_eq!(
        decode_at_index(String::from("leet2code3"), 10),
        String::from("o")
    );
    assert_eq!(decode_at_index(String::from("ha22"), 5), String::from("h"));
    assert_eq!(
        decode_at_index(String::from("a2345678999999999999999"), 1),
        String::from("a")
    );
}
