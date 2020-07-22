// https://leetcode.com/problems/invalid-transactions/
pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
    todo!()
}
// array string
#[test]
#[ignore]
fn test2_1169() {
    assert_eq!(
        invalid_transactions(vec![
            String::from("alice,20,800,mtv"),
            String::from("alice,50,100,beijing")
        ]),
        vec![
            String::from("alice,20,800,mtv"),
            String::from("alice,50,100,beijing")
        ]
    );
    assert_eq!(
        invalid_transactions(vec![
            String::from("alice,20,800,mtv"),
            String::from("alice,50,1200,mtv")
        ]),
        vec![String::from("alice,50,1200,mtv")]
    );
    assert_eq!(
        invalid_transactions(vec![
            String::from("alice,20,800,mtv"),
            String::from("bob,50,1200,mtv")
        ]),
        vec![String::from("bob,50,1200,mtv")]
    );
}
