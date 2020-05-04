// https://leetcode.com/problems/rank-teams-by-votes/
pub fn rank_teams(votes: Vec<String>) -> String {
    todo!()
}
// sort array
#[test]
#[ignore]
fn test2_1366() {
    assert_eq!(
        rank_teams(vec![
            String::from("ABC"),
            String::from("ACB"),
            String::from("ABC"),
            String::from("ACB"),
            String::from("ACB")
        ]),
        String::from("ACB")
    );
    assert_eq!(
        rank_teams(vec![String::from("WXYZ"), String::from("XYZW")]),
        String::from("XWYZ")
    );
    assert_eq!(
        rank_teams(vec![String::from("ZMNAGUEDSJYLBOPHRQICWFXTVK")]),
        String::from("ZMNAGUEDSJYLBOPHRQICWFXTVK")
    );
    assert_eq!(
        rank_teams(vec![
            String::from("BCA"),
            String::from("CAB"),
            String::from("CBA"),
            String::from("ABC"),
            String::from("ACB"),
            String::from("BAC")
        ]),
        String::from("ABC")
    );
    assert_eq!(
        rank_teams(vec![
            String::from("M"),
            String::from("M"),
            String::from("M"),
            String::from("M")
        ]),
        String::from("M")
    );
}
