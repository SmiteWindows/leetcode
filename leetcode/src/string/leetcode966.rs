// https://leetcode-cn.com/problems/vowel-spellchecker/
// Runtime: 40 ms
// Memory Usage: 4.1 MB
use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::Hasher,
};
pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
    let dict = Dict::new(wordlist);
    queries.into_iter().map(|s| dict.query(s)).collect()
}

struct Dict {
    same: HashSet<String>,
    capitlization: HashMap<u64, String>,
    vowel_error: HashMap<u64, String>,
}

impl Dict {
    fn new(wordlist: Vec<String>) -> Self {
        let mut same = HashSet::new();
        let mut capitlization = HashMap::new();
        let mut vowel_error = HashMap::new();

        for s in wordlist.iter().rev() {
            same.insert(s.to_string());
            capitlization.insert(Self::hash1(&s), s.to_string());
            vowel_error.insert(Self::hash2(&s), s.to_string());
        }
        Self {
            same,
            capitlization,
            vowel_error,
        }
    }

    fn query(&self, s: String) -> String {
        if self.same.contains(&s) {
            return s;
        }
        if let Some(res) = self.capitlization.get(&Self::hash1(&s)) {
            return res.to_string();
        }
        if let Some(res) = self.vowel_error.get(&Self::hash2(&s)) {
            return res.to_string();
        }
        "".to_string()
    }

    fn hash1(s: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        for c in s.to_lowercase().chars() {
            hasher.write_u8(c as u8);
        }
        hasher.finish()
    }

    fn hash2(s: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        for c in s.to_lowercase().chars() {
            let c = match c {
                'a' | 'e' | 'i' | 'o' | 'u' => '_',
                _ => c,
            };
            hasher.write_u8(c as u8);
        }
        hasher.finish()
    }
}
// hash_table string
#[test]
fn test2_966() {
    assert_eq!(
        spellchecker(
            vec![
                String::from("KiTe"),
                String::from("kite"),
                String::from("hare"),
                String::from("Hare")
            ],
            vec![
                String::from("kite"),
                String::from("Kite"),
                String::from("KiTe"),
                String::from("Hare"),
                String::from("HARE"),
                String::from("Hear"),
                String::from("hear"),
                String::from("keti"),
                String::from("keet"),
                String::from("keto")
            ]
        ),
        vec![
            String::from("kite"),
            String::from("KiTe"),
            String::from("KiTe"),
            String::from("Hare"),
            String::from("hare"),
            String::from(""),
            String::from(""),
            String::from("KiTe"),
            String::from(""),
            String::from("KiTe")
        ]
    );
}
