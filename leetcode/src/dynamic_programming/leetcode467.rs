// https://leetcode-cn.com/problems/unique-substrings-in-wraparound-string/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn find_substring_in_wrapround_string(p: String) -> i32 {
    let mut count = [0_usize; 26];
    let p = p.bytes().collect::<Vec<u8>>();
    let n = p.len();
    for &pi in p.iter().take(n) {
        let j = (pi - b'a') as usize;
        count[j] = 1;
    }
    let mut l = 1;
    for i in 1..n {
        let j = (p[i] - b'a') as usize;
        let k = (p[i - 1] - b'a') as usize;
        l = if (k + 1) % 26 == j { l + 1 } else { 1 };
        count[j] = count[j].max(l);
    }
    let res = count.iter().sum::<usize>();
    res as i32
}
// dynamic_programming
#[test]
fn test1_467() {
    assert_eq!(find_substring_in_wrapround_string("a".to_string()), 1);
    assert_eq!(find_substring_in_wrapround_string("cac".to_string()), 2);
    assert_eq!(find_substring_in_wrapround_string("zab".to_string()), 6);
}
