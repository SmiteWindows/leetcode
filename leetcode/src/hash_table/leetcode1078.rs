// https://leetcode-cn.com/problems/occurrences-after-bigram/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
    let mut res = vec![];
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
    assert_eq!(
        find_ocurrences(
            String::from("alice is a good girl she is a good student"),
            String::from("a"),
            String::from("good")
        ),
        vec![String::from("girl"), String::from("student")]
    );
    assert_eq!(
        find_ocurrences(
            String::from("we will we will rock you"),
            String::from("we"),
            String::from("will")
        ),
        vec![String::from("we"), String::from("rock")]
    );
}
