// https://leetcode.com/problems/subdomain-visit-count/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
    fn parse(s: String) -> (Vec<String>, usize) {
        let mut domains = vec![];
        let mut iter = s.split_whitespace();
        let count = iter.next().unwrap().parse::<usize>().unwrap();
        let domain = iter.next().unwrap().parse::<String>().unwrap();
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
    use leetcode_prelude::{assert_eq_sorted, vec_string};
    assert_eq_sorted!(
        subdomain_visits(vec_string!["9001 discuss.leetcode.com"]),
        vec_string!["9001 discuss.leetcode.com", "9001 leetcode.com", "9001 com"]
    );
    assert_eq_sorted!(
        subdomain_visits(vec_string![
            "900 google.mail.com",
            "50 yahoo.com",
            "1 intel.mail.com",
            "5 wiki.org"
        ]),
        vec_string![
            "901 mail.com",
            "50 yahoo.com",
            "900 google.mail.com",
            "5 wiki.org",
            "5 org",
            "1 intel.mail.com",
            "951 com"
        ]
    );
}
