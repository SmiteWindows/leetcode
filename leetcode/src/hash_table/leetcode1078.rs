// https://leetcode-cn.com/problems/occurrences-after-bigram/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
    let mut res = Vec::new();
    let words = text.split_whitespace().collect::<Vec<_>>();
    words.windows(3).for_each(|v| {
        if v[0] == first && v[1] == second {
            res.push(v[2].to_string());
        }
    });
    res
}
// hash_table
#[test]
fn test1_1078() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        find_ocurrences(
            "alice is a good girl she is a good student".to_string(),
            "a".to_string(),
            "good".to_string()
        ),
        vec_string!["girl", "student"]
    );
    assert_eq!(
        find_ocurrences(
            "we will we will rock you".to_string(),
            "we".to_string(),
            "will".to_string()
        ),
        vec_string!["we", "rock"]
    );
}
