// https://leetcode-cn.com/problems/unique-email-addresses/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
use std::collections::HashSet;
pub fn num_unique_emails(emails: Vec<String>) -> i32 {
    let mut hs = HashSet::new();
    let emails = emails
        .iter()
        .map(|email| Email::new(email.to_string()))
        .collect::<Vec<_>>();
    for email in emails {
        hs.insert(email);
    }
    hs.len() as i32
}

#[derive(PartialEq, Eq, Hash)]
struct Email {
    local_name: String,
    domain_name: String,
}

impl Email {
    fn new(s: String) -> Self {
        let mut iter = s.split('@');
        let left = iter.next().unwrap().to_string();
        let domain_name = iter.next().unwrap().to_string();
        let mut local_name = left.split('+').next().unwrap().to_string();
        local_name.retain(|c| c != '.');
        Self {
            local_name,
            domain_name,
        }
    }
}

// string
#[test]
fn test1_929() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        num_unique_emails(vec_string![
            "test.email+alex@leetcode.com",
            "test.e.mail+bob.cathy@leetcode.com",
            "testemail+david@lee.tcode.com"
        ]),
        2
    );
}
