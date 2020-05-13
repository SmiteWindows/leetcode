// https://leetcode.com/problems/unique-email-addresses/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
use std::collections::HashSet;
#[derive(PartialEq, Eq, Hash)]
struct Email {
    local_name: String,
    domain_name: String,
}

impl Email {
    fn new(s: String) -> Self {
        let mut iter = s.split('@');
        let left: String = iter.next().unwrap().to_string();
        let domain_name: String = iter.next().unwrap().to_string();
        let mut local_name: String = left.split('+').next().unwrap().to_string();
        local_name.retain(|c| c != '.');
        Email {
            local_name,
            domain_name,
        }
    }
}

pub fn num_unique_emails(emails: Vec<String>) -> i32 {
    let mut hs: HashSet<Email> = HashSet::new();
    let emails: Vec<Email> = emails
        .iter()
        .map(|email| Email::new(email.to_string()))
        .collect();
    for email in emails {
        hs.insert(email);
    }
    hs.len() as i32
}
// string
#[test]
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
