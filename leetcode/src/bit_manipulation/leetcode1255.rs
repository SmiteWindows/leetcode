// https://leetcode-cn.com/problems/maximum-score-words-formed-by-letters/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
    let mut count = vec![0; 26];
    for c in letters {
        count[(c as u8 - b'a') as usize] += 1;
    }
    let n = words.len();
    let scores: Vec<i32> = words
        .iter()
        .map(|s| s.bytes().map(|b| score[(b - b'a') as usize]).sum())
        .collect();
    let mut res = 0;
    dfs(0, 0, &mut count, &mut res, &words, &scores, n);
    res
}

fn dfs(
    start: usize,
    sum: i32,
    count: &mut Vec<i32>,
    max: &mut i32,
    words: &[String],
    scores: &[i32],
    n: usize,
) {
    if start == n {
        *max = (*max).max(sum);
    } else {
        dfs(start + 1, sum, count, max, words, scores, n);
        update(count, &words[start], -1);
        if check(count, &words[start]) {
            dfs(start + 1, sum + scores[start], count, max, words, scores, n);
        }
        update(count, &words[start], 1);
    }
}

fn update(count: &mut Vec<i32>, s: &str, val: i32) {
    for c in s.chars() {
        count[(c as u8 - b'a') as usize] += val;
    }
}

fn check(count: &mut Vec<i32>, s: &str) -> bool {
    for c in s.chars() {
        if count[(c as u8 - b'a') as usize] < 0 {
            return false;
        }
    }
    true
}
// bit_manipulation
#[test]
fn test1_1255() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        max_score_words(
            vec_string!["dog", "cat", "dad", "good"],
            vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
            vec![1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ),
        23
    );
    assert_eq!(
        max_score_words(
            vec_string!["xxxz", "ax", "bx", "cx"],
            vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'],
            vec![4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10]
        ),
        27
    );
    assert_eq!(
        max_score_words(
            vec_string!["leetcode"],
            vec!['l', 'e', 't', 'c', 'o', 'd'],
            vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]
        ),
        0
    );
}
