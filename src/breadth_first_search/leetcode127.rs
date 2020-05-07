// https://leetcode.com/problems/word-ladder/
pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    todo!()
}
// breadth_first_search
#[test]
#[ignore]
fn test1_127() {
    assert_eq!(
        ladder_length(
            String::from("hit"),
            String::from("cog"),
            vec![
                String::from("hot"),
                String::from("dot"),
                String::from("dog"),
                String::from("lot"),
                String::from("log"),
                String::from("cog")
            ]
        ),
        5
    );
    assert_eq!(
        ladder_length(
            String::from("hit"),
            String::from("cog"),
            vec![
                String::from("hot"),
                String::from("dot"),
                String::from("dog"),
                String::from("lot"),
                String::from("log")
            ]
        ),
        0
    );
}
