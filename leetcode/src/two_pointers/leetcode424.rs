// https://leetcode-cn.com/problems/longest-repeating-character-replacement/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut count = vec![0; 26];
    let n = s.len();
    let s = s.chars().collect::<Vec<_>>();
    let mut start = 0;
    let mut end = 0;
    let mut res = 0;
    while end < n {
        if sum(&count) <= k {
            count[(s[end] as u8 - b'A') as usize] += 1;
            end += 1;
        } else {
            count[(s[start] as u8 - b'A') as usize] -= 1;
            start += 1;
        }
        if sum(&count) <= k {
            res = res.max(end - start);
        }
    }
    res as i32
}

fn sum(count: &[i32]) -> i32 {
    let max = count.iter().copied().max().unwrap();
    count.iter().sum::<i32>() - max
}
// two_pointers sliding_window
#[test]
fn test2_424() {
    assert_eq!(character_replacement("ABAB".to_string(), 2), 4);
    assert_eq!(character_replacement("AABABBA".to_string(), 1), 4);
}
