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
        decode_at_index("leet2code3".to_string(), 10),
        "o".to_string()
    );
    assert_eq!(decode_at_index("ha22".to_string(), 5), "h".to_string());
    assert_eq!(
        decode_at_index("a2345678999999999999999".to_string(), 1),
        "a".to_string()
    );
}
