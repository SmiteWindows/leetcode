// https://leetcode.com/problems/subdomain-visit-count/
pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_811() {
    assert_eq!(
        subdomain_visits(vec![String::from("9001 discuss.leetcode.com")]),
        vec![
            String::from("9001 discuss.leetcode.com"),
            String::from("9001 leetcode.com"),
            String::from("9001 com")
        ]
    );
    assert_eq!(
        subdomain_visits(vec![
            String::from("900 google.mail.com"),
            String::from("50 yahoo.com"),
            String::from("1 intel.mail.com"),
            String::from("5 wiki.org")
        ]),
        vec![
            String::from("901 mail.com"),
            String::from("50 yahoo.com"),
            String::from("900 google.mail.com"),
            String::from("5 wiki.org"),
            String::from("5 org"),
            String::from("1 intel.mail.com"),
            String::from("951 com")
        ]
    );
}
