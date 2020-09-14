// https://leetcode-cn.com/problems/can-make-palindrome-from-substring/
pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
    todo!()
}
// array string
#[test]
#[ignore]
fn test2_1177() {
    assert_eq!(
        can_make_pali_queries(
            "abcda".to_string(),
            vec![
                vec![3, 3, 0],
                vec![1, 2, 0],
                vec![0, 3, 1],
                vec![0, 3, 2],
                vec![0, 4, 1]
            ]
        ),
        vec![true, false, false, true, true]
    );
}
