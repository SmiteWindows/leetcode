// https://leetcode.com/problems/utf-8-validation/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn valid_utf8(data: Vec<i32>) -> bool {
    let mut count = 0;
    for x in data {
        if count == 0 {
            if x >> 3 & 0b11111 == 0b11110 {
                count += 3;
                continue;
            }
            if x >> 4 & 0b1111 == 0b1110 {
                count += 2;
                continue;
            }
            if x >> 5 & 0b111 == 0b110 {
                count += 1;
                continue;
            }
            if x >> 7 & 0b1 == 0 {
                continue;
            }
            return false;
        } else {
            if x >> 6 & 0b11 == 0b10 {
                count -= 1;
                continue;
            }
            return false;
        }
    }
    count == 0
}
// bit_manipulation
#[test]
fn test1_393() {
    assert_eq!(valid_utf8(vec![197, 130, 1]), true);
    assert_eq!(valid_utf8(vec![235, 140, 4]), false);
}
