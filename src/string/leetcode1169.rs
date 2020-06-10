// https://leetcode.com/problems/invalid-transactions/
// Runtime: 8 ms
// Memory Usage: 2.4 MB
use std::collections::{HashMap, HashSet};
pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
    let n = transactions.len();
    let mut hm: HashMap<String, Vec<(i32, String, String)>> = HashMap::new();
    let mut hs = HashSet::new();
    for i in 0..n {
        let transaction = &transactions[i];
        let mut v = transaction
            .split_terminator(',')
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let city = v.pop().unwrap();
        let amount = v.pop().unwrap().parse::<i32>().unwrap();
        let time = v.pop().unwrap().parse::<i32>().unwrap();
        let name = v.pop().unwrap();
        hm.entry(name)
            .or_default()
            .push((time, city, transaction.to_string()));
        if amount > 1000 {
            hs.insert(transactions[i].to_string());
        }
    }
    for transactions in hm.values() {
        let n = transactions.len();
        for i in 0..n {
            for j in i + 1..n {
                let ti = &transactions[i];
                let tj = &transactions[j];
                if (ti.0 - tj.0).abs() <= 60 && ti.1 != tj.1 {
                    hs.insert(ti.2.to_string());
                    hs.insert(tj.2.to_string());
                }
            }
        }
    }
    hs.drain().collect()
}
// array string
#[test]
fn test1_1169() {
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
