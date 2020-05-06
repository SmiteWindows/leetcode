// https://leetcode.com/problems/shortest-completing-word/
pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_748() {
    assert_eq!(
        shortest_completing_word(
            String::from("1s3 PSt"),
            vec![
                String::from("step"),
                String::from("steps"),
                String::from("stripe"),
                String::from("stepple")
            ]
        ),
        String::from("steps")
    );
    assert_eq!(
        shortest_completing_word(
            String::from("1s3 456"),
            vec![
                String::from("looks"),
                String::from("pest"),
                String::from("stew"),
                String::from("show")
            ]
        ),
        String::from("pest")
    );
}
