// https://leetcode.com/problems/concatenated-words/
pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
    todo!()
}
// dynamic_programming depth_first_search trie
#[test]
#[ignore]
fn test2_472() {
    assert_eq!(
        find_all_concatenated_words_in_a_dict(vec![
            String::from("cat"),
            String::from("cats"),
            String::from("catsdogcats"),
            String::from("dog"),
            String::from("dogcatsdog"),
            String::from("hippopotamuses"),
            String::from("rat"),
            String::from("ratcatdogcat"),
        ]),
        vec![
            String::from("catsdogcats"),
            String::from("dogcatsdog"),
            String::from("ratcatdogcat"),
        ],
    );
}
