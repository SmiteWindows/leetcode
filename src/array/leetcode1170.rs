// https://leetcode.com/problems/compare-strings-by-frequency-of-the-smallest-character/
pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
    todo!()
}
// array string
#[test]
#[ignore]
fn test2_1170() {
    assert_eq!(
        num_smaller_by_frequency(vec![String::from("cbd")], vec![String::from("zaaaz")]),
        vec![1]
    );
    assert_eq!(
        num_smaller_by_frequency(
            vec![String::from("bbb"), String::from("cc")],
            vec![
                String::from("a"),
                String::from("aa"),
                String::from("aaa"),
                String::from("aaaa")
            ]
        ),
        vec![1, 2]
    );
}
