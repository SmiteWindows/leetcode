// https://leetcode-cn.com/problems/find-common-characters/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn common_chars(a: Vec<String>) -> Vec<String> {
    let n = a.len();
    let mut counts = vec![vec![0; 256]; n];
    for (i, w) in a.iter().enumerate() {
        for c in w.chars() {
            counts[i][c as usize] += 1;
        }
    }
    let mut res = Vec::new();
    for i in 0..26 {
        let c: u8 = b'a' + i;
        let mut min = usize::MAX;
        for ch in counts.iter() {
            let count = ch[c as usize];
            min = usize::min(count, min);
        }
        for _ in 0..min {
            res.push(format!("{}", c as char))
        }
    }
    res
}
// hash_table array
#[test]
fn test2_1002() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        common_chars(vec_string!["bella", "label", "roller"]),
        vec_string!["e", "l", "l"]
    );
    assert_eq!(
        common_chars(vec_string!["cool", "lock", "cook"]),
        vec_string!["c", "o"]
    );
}
