// https://leetcode-cn.com/problems/stream-of-characters/
// Runtime: 104 ms
// Memory Usage: 20.6 MB
use std::collections::HashMap;
struct StreamChecker {
    trie: Trie,
    stream: Vec<char>,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::new();
        for s in words {
            trie.insert(s);
        }
        let stream = vec![];
        Self { trie, stream }
    }

    fn query(&mut self, letter: char) -> bool {
        self.stream.push(letter);
        self.trie.search(&self.stream)
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut link = self;
        for c in word.chars().rev() {
            link = link.children.entry(c).or_default();
        }
        link.end = true;
    }

    fn search(&self, stream: &[char]) -> bool {
        let mut link = self;
        for c in stream.iter().rev() {
            if let Some(next) = link.children.get(&c) {
                link = next;
                if next.end {
                    return true;
                }
            } else {
                return false;
            }
        }
        false
    }
}
/**
 * Your StreamChecker object will be instantiated and called as such:
 * let obj = StreamChecker::new(words);
 * let ret_1: bool = obj.query(letter);
 */
// trie
#[test]
fn test1_1032() {
    use leetcode_prelude::vec_string;
    let words = vec_string!["cd", "f", "kl"];
    let mut obj = StreamChecker::new(words);
    assert_eq!(obj.query('a'), false);
    assert_eq!(obj.query('b'), false);
    assert_eq!(obj.query('c'), false);
    assert_eq!(obj.query('d'), true);
    assert_eq!(obj.query('e'), false);
    assert_eq!(obj.query('f'), true);
    assert_eq!(obj.query('g'), false);
    assert_eq!(obj.query('h'), false);
    assert_eq!(obj.query('i'), false);
    assert_eq!(obj.query('j'), false);
    assert_eq!(obj.query('k'), false);
    assert_eq!(obj.query('l'), true);
}
