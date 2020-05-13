// https://leetcode.com/problems/find-common-characters/
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
    let mut res = vec![];
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
    assert_eq!(
        common_chars(vec![
            String::from("bella"),
            String::from("label"),
            String::from("roller")
        ]),
        vec![String::from("e"), String::from("l"), String::from("l")]
    );
    assert_eq!(
        common_chars(vec![
            String::from("cool"),
            String::from("lock"),
            String::from("cook")
        ]),
        vec![String::from("c"), String::from("o")]
    );
}
