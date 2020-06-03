// https://leetcode.com/problems/subdomain-visit-count/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
    fn parse(s: String) -> (Vec<String>, usize) {
        let mut domains: Vec<String> = vec![];
        let mut iter = s.split_whitespace();
        let count: usize = iter.next().expect("exist").parse::<usize>().expect("exist");
        let domain: String = iter
            .next()
            .expect("exist")
            .parse::<String>()
            .expect("exist");
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
fn test1_811() {
    let mut res1 = subdomain_visits(vec![String::from("9001 discuss.leetcode.com")]);
    res1.sort();
    assert_eq!(
        res1,
        vec![
            String::from("9001 com"),
            String::from("9001 discuss.leetcode.com"),
            String::from("9001 leetcode.com")
        ]
    );
    let mut res2 = subdomain_visits(vec![
        String::from("900 google.mail.com"),
        String::from("50 yahoo.com"),
        String::from("1 intel.mail.com"),
        String::from("5 wiki.org"),
    ]);
    res2.sort();
    assert_eq!(
        res2,
        vec![
            String::from("1 intel.mail.com"),
            String::from("5 org"),
            String::from("5 wiki.org"),
            String::from("50 yahoo.com"),
            String::from("900 google.mail.com"),
            String::from("901 mail.com"),
            String::from("951 com")
        ]
    );
}
