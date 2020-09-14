// https://leetcode-cn.com/problems/compare-strings-by-frequency-of-the-smallest-character/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
    let f_words = words.iter().map(|s| f(s)).collect::<Vec<_>>();
    let mut counts = vec![0; 12];
    for f in f_words {
        counts[f] += 1;
    }
    for i in (1..10).rev() {
        counts[i] += counts[i + 1];
    }
    queries
        .iter()
        .map(|s| f(s))
        .map(|f| counts[f + 1])
        .collect()
}

fn f(s: &str) -> usize {
    let mut count = vec![0; 26];
    let mut min = b'z';
    for b in s.bytes() {
        min = min.min(b);
        count[(b - b'a') as usize] += 1;
    }
    count[(min - b'a') as usize]
}
// array string
#[test]
fn test1_1170() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        num_smaller_by_frequency(vec_string!["cbd"], vec_string!["zaaaz"]),
        vec![1]
    );
    assert_eq!(
        num_smaller_by_frequency(
            vec_string!["bbb", "cc"],
            vec_string!["a", "aa", "aaa", "aaaa"]
        ),
        vec![1, 2]
    );
}
