// https://leetcode.com/problems/jewels-and-stones/
pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_771() {
    assert_eq!(
        num_jewels_in_stones(String::from("aA"), String::from("aAAbbbb")),
        3
    );
    assert_eq!(
        num_jewels_in_stones(String::from("z"), String::from("ZZ")),
        0
    );
}
