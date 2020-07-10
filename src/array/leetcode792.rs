// https://leetcode.com/problems/number-of-matching-subsequences/
// Runtime: 16 ms
// Memory Usage: 2.8 MB
pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
    let mut queues = vec![vec![]; 26];
    let mut temp = vec![];
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
    assert_eq!(
        num_matching_subseq(
            String::from("abcde"),
            vec![
                String::from("a"),
                String::from("bb"),
                String::from("acd"),
                String::from("ace")
            ]
        ),
        3
    );
}
