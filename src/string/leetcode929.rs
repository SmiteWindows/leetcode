// https://leetcode.com/problems/unique-email-addresses/
pub fn num_unique_emails(emails: Vec<String>) -> i32 {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_929() {
    assert_eq!(
        num_unique_emails(vec![
            String::from("test.email+alex@leetcode.com"),
            String::from("test.e.mail+bob.cathy@leetcode.com"),
            String::from("testemail+david@lee.tcode.com")
        ]),
        2
    );
}
