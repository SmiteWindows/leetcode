// https://leetcode-cn.com/problems/implement-magic-dictionary/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
#[derive(Default)]
struct MagicDictionary {
    data: HashMap<Vec<char>, Vec<(char, usize)>>,
}

impl MagicDictionary {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self::default()
    }

    /** Build a dictionary through a list of words */
    fn build_dict(&mut self, dict: Vec<String>) {
        for s in dict {
            let s = s.chars().collect::<Vec<_>>();
            let n = s.len();
            for i in 0..n {
                let mut t = vec![];
                for &c in &s[..i] {
                    t.push(c);
                }
                for &c in &s[i + 1..] {
                    t.push(c);
                }
                self.data.entry(t).or_default().push((s[i], i));
            }
        }
    }

    /** Returns if there is any word in the trie that equals to the given word after modifying exactly one character */
    fn search(&self, word: String) -> bool {
        let s = word.chars().collect::<Vec<_>>();
        let n = s.len();
        for i in 0..n {
            let mut t = vec![];
            for &c in &s[..i] {
                t.push(c);
            }
            for &c in &s[i + 1..] {
                t.push(c);
            }
            if let Some(v) = self.data.get(&t) {
                for &(c, j) in v {
                    if c != s[i] && i == j {
                        return true;
                    }
                }
            }
        }
        false
    }
}
/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dict);
 * let ret_2: bool = obj.search(word);
 */
// hash_table trie
#[test]
fn test2_676() {
    let mut obj = MagicDictionary::new();
    obj.build_dict(vec!["hello".to_string(), "leetcode".to_string()]);
    assert_eq!(obj.search("hello".to_string()), false);
    assert_eq!(obj.search("hhllo".to_string()), true);
    assert_eq!(obj.search("hell".to_string()), false);
    assert_eq!(obj.search("leetcoded".to_string()), false);
}
