// https://leetcode-cn.com/problems/number-of-substrings-containing-all-three-characters/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn number_of_substrings(s: String) -> i32 {
    let mut count = [0; 3];
    let s = s.bytes().collect::<Vec<_>>();
    let n = s.len();
    let mut j = 0;
    let mut res = 0;
    for i in 0..n {
        count[(s[i] - b'a') as usize] += 1;
        while count[0] > 0 && count[1] > 0 && count[2] > 0 {
            count[(s[j] - b'a') as usize] -= 1;
            j += 1;
        }
        res += j;
    }
    res as i32
}
// string
#[test]
fn test1_1358() {
    assert_eq!(number_of_substrings("abcabc".to_string()), 10);
    assert_eq!(number_of_substrings("aaacb".to_string()), 3);
    assert_eq!(number_of_substrings("abc".to_string()), 1);
}
