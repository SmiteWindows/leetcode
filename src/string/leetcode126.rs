// https://leetcode.com/problems/word-ladder-ii/
pub fn find_ladders(
    begin_word: String,
    end_word: String,
    word_list: Vec<String>,
) -> Vec<Vec<String>> {
    todo!()
}
// string backtracking array breadth_first_search
#[test]
#[ignore]
fn test4_126() {
    assert_eq!(
        find_ladders(
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
        vec![
            vec![
                String::from("hit"),
                String::from("hot"),
                String::from("dot"),
                String::from("dog"),
                String::from("cog")
            ],
            vec![
                String::from("hit"),
                String::from("hot"),
                String::from("lot"),
                String::from("log"),
                String::from("cog")
            ]
        ]
    );
    assert_eq!(
        find_ladders(
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
        vec![vec![String::new()]]
    );
}
