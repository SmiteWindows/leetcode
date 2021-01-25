// https://leetcode-cn.com/problems/word-subsets/
// Runtime: 36 ms
// Memory Usage: 3.3 MB
pub fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
    let mut max_count = [0; 26];
    for s in b {
        let mut count = [0; 26];
        for b in s.bytes() {
            count[(b - b'a') as usize] += 1;
        }
        for i in 0..26 {
            max_count[i] = max_count[i].max(count[i]);
        }
    }
    let mut res = vec![];
    'a: for s in a {
        let mut count = [0; 26];
        for b in s.bytes() {
            count[(b - b'a') as usize] += 1;
        }
        for i in 0..26 {
            if count[i] < max_count[i] {
                continue 'a;
            };
        }
        res.push(s)
    }
    res
}
// string
#[test]
fn test1_916() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        word_subsets(
            vec_string!["amazon", "apple", "facebook", "google", "leetcode"],
            vec_string!["e", "o"]
        ),
        vec_string!["facebook", "google", "leetcode"]
    );
    assert_eq!(
        word_subsets(
            vec_string!["amazon", "apple", "facebook", "google", "leetcode"],
            vec_string!["l", "e"]
        ),
        vec_string!["apple", "google", "leetcode"]
    );
    assert_eq!(
        word_subsets(
            vec_string!["amazon", "apple", "facebook", "google", "leetcode"],
            vec_string!["e", "oo"]
        ),
        vec_string!["facebook", "google"]
    );
    assert_eq!(
        word_subsets(
            vec_string!["amazon", "apple", "facebook", "google", "leetcode"],
            vec_string!["lo", "eo"]
        ),
        vec_string!["google", "leetcode"]
    );
    assert_eq!(
        word_subsets(
            vec_string!["amazon", "apple", "facebook", "google", "leetcode"],
            vec_string!["ec", "oc", "ceo"]
        ),
        vec_string!["facebook", "leetcode"]
    );
}
