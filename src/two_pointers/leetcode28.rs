// https://leetcode.com/problems/implement-strstr/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack: Vec<char> = haystack.chars().collect();
    let needle: Vec<char> = needle.chars().collect();
    let h_len = haystack.len();
    let n_len = needle.len();
    if n_len == 0 {
        return 0;
    }
    if h_len < n_len {
        return -1;
    }
    let mut pn = 0;
    while pn < h_len - n_len + 1 {
        while pn < h_len - n_len + 1 && haystack[pn] != needle[0] {
            pn += 1;
        }
        let mut cur_len = 0;
        let mut pl = 0;
        while pl < n_len && pn < h_len && haystack[pn] == needle[pl] {
            pn += 1;
            pl += 1;
            cur_len += 1;
        }
        if cur_len == n_len {
            return (pn - n_len) as i32;
        }
        pn = pn - cur_len + 1;
    }
    -1
}
// string two_pointers
#[test]
fn test1_28() {
    assert_eq!(str_str(String::from("hello"), String::from("ll")), 2);
    assert_eq!(str_str(String::from("aaaaa"), String::from("bba")), -1);
    assert_eq!(str_str(String::from("abb"), String::from("abaaa")), -1);
}
