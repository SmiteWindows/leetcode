// https://leetcode.com/problems/bulls-and-cows/
pub fn get_hint(secret: String, guess: String) -> String {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_299() {
    assert_eq!(
        get_hint(String::from("1807"), String::from("7810")),
        String::from("1A3B")
    );
    assert_eq!(
        get_hint(String::from("1123"), String::from("0111")),
        String::from("1A1B")
    );
}
