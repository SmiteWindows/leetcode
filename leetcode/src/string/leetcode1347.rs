// https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn min_steps(s: String, t: String) -> i32 {
    let mut count: Vec<i32> = vec![0; 26];
    for c in s.chars() {
        let i = (c as u32 - 'a' as u32) as usize;
        count[i] += 1;
    }
    for c in t.chars() {
        let i = (c as u32 - 'a' as u32) as usize;
        count[i] -= 1;
    }
    count.iter().map(|&x| x.abs()).sum::<i32>() / 2
}
// string
#[test]
fn test1_1347() {
    assert_eq!(min_steps(String::from("bab"), String::from("aba")), 1);
    assert_eq!(
        min_steps(String::from("leetcode"), String::from("practice")),
        5
    );
    assert_eq!(
        min_steps(String::from("anagram"), String::from("mangaar")),
        0
    );
    assert_eq!(min_steps(String::from("xxyyzz"), String::from("xxyyzz")), 0);
    assert_eq!(min_steps(String::from("friend"), String::from("family")), 4);
}
