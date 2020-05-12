// https://leetcode.com/problems/subdomain-visit-count/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
    fn parse(s: String) -> (Vec<String>, usize) {
        let mut domains: Vec<String> = vec![];
        let mut iter = s.split_whitespace();
        let count: usize = iter.next().unwrap().parse::<usize>().unwrap();
        let domain: String = iter.next().unwrap().parse::<String>().unwrap();
        for (i, c) in domain.chars().enumerate() {
            if c == '.' {
                let subdomain = &domain[i + 1..];
                domains.push(subdomain.to_string());
            }
        }
        domains.push(domain);
        (domains, count)
    }

    let mut res = vec![];
    let mut hm: HashMap<String, usize> = HashMap::new();
    for s in cpdomains {
        let (domains, count) = parse(s);
        for domain in domains {
            *hm.entry(domain).or_default() += count;
        }
    }
    for (domain, count) in hm {
        res.push(format!("{} {}", count, domain));
    }
    res
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
