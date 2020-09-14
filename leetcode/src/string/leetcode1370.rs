// https://leetcode-cn.com/problems/increasing-decreasing-string/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn sort_string(s: String) -> String {
    let mut count: Vec<usize> = vec![0; 26];
    let mut n = s.len();
    for c in s.chars() {
        count[(c as u8 - b'a') as usize] += 1;
    }
    let mut direction = true;
    let mut res = "".to_string();
    while n > 0 {
        if direction {
            for (i, val) in count.iter_mut().enumerate().take(26) {
                if *val > 0 {
                    *val -= 1;
                    n -= 1;
                    res.push((b'a' + i as u8) as char);
                }
            }
        } else {
            for (i, val) in count.iter_mut().enumerate().take(26).rev() {
                if *val > 0 {
                    *val -= 1;
                    n -= 1;
                    res.push((b'a' + i as u8) as char);
                }
            }
        }
        direction = !direction;
    }
    res
}
// sort string
#[test]
fn test2_1370() {
    assert_eq!(
        sort_string("aaaabbbbcccc")),
        "abccbaabccba")
    );
    assert_eq!(sort_string("rat")), "art"));
    assert_eq!(
        sort_string("leetcode")),
        "cdelotee")
    );
    assert_eq!(
        sort_string("ggggggg")),
        "ggggggg")
    );
    assert_eq!(sort_string("spo")), "ops"));
}
