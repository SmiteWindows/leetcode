// https://leetcode.com/problems/dota2-senate/
pub fn predict_party_victory(senate: String) -> String {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_649() {
    assert_eq!(
        predict_party_victory(String::from("RD")),
        String::from("Radiant")
    );
    assert_eq!(
        predict_party_victory(String::from("RDD")),
        String::from("Dire")
    );
}
