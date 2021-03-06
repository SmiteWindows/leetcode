// https://leetcode-cn.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/
// Runtime: 4 ms
// Memory Usage: 3.3 MB
pub fn has_all_codes(s: String, k: i32) -> bool {
    let k = k as usize;
    let m = 1 << k;
    let mut set = vec![false; m];
    let mut x = 0;
    let sn = s.len();
    let mask = m as u32 - 1;
    for (index, c) in s.char_indices().rev() {
        x <<= 1;
        x |= (c as u8 - b'0') as u32;
        x &= mask;
        if index + k <= sn {
            set[x as usize] = true;
        }
    }
    set.iter().all(|&b| b)
}
// string bit_manipulation
#[test]
fn test2_1461() {
    assert_eq!(has_all_codes("00110110".to_string(), 2), true);
    assert_eq!(has_all_codes("00110".to_string(), 2), true);
    assert_eq!(has_all_codes("0110".to_string(), 1), true);
    assert_eq!(has_all_codes("0110".to_string(), 2), false);
    assert_eq!(has_all_codes("0000000001011100".to_string(), 4), false);
}
