// https://leetcode.com/problems/count-binary-substrings/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn count_binary_substrings(s: String) -> i32 {
    let mut prev: usize = 0;
    let mut curr: usize = 0;
    let mut curr_c: Option<char> = None;
    let mut res = 0;
    for c in s.chars() {
        if let Some(cc) = curr_c {
            if cc == c {
                curr += 1;
            } else {
                res += prev.min(curr);
                curr_c = Some(c);
                prev = curr;
                curr = 1;
            }
        } else {
            curr_c = Some(c);
            curr = 1;
        }
    }
    res += prev.min(curr);
    res as i32
}
// string
#[test]
fn test1_696() {
    assert_eq!(count_binary_substrings(String::from("00110011")), 6);
    assert_eq!(count_binary_substrings(String::from("10101")), 4);
}
