// https://leetcode-cn.com/problems/number-of-matching-subsequences/
// Runtime: 16 ms
// Memory Usage: 2.8 MB
pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
    let mut queues = vec![Vec::new(); 26];
    let mut temp = Vec::new();
    for word in &words {
        let mut iter = word.chars();
        if let Some(c) = iter.next() {
            queues[(c as u8 - b'a') as usize].push(iter);
        }
    }
    let mut res = 0;
    for c in s.chars() {
        let iters = &mut queues[(c as u8 - b'a') as usize];
        while let Some(mut iter) = iters.pop() {
            if let Some(d) = iter.next() {
                temp.push((d, iter));
            } else {
                res += 1;
            }
        }
        while let Some((c, iter)) = temp.pop() {
            queues[(c as u8 - b'a') as usize].push(iter);
        }
    }
    res
}
// array
#[test]
fn test1_792() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        num_matching_subseq("abcde".to_string(), vec_string!["a", "bb", "acd", "ace"]),
        3
    );
}
