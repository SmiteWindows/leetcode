// https://leetcode.com/problems/word-search-ii/
// Runtime: 24 ms
// Memory Usage: 12.6 MB
use std::collections::HashMap;
pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let mut board = board;
    let mut trie = Trie::default();
    for word in words {
        trie.insert(word);
    }
    let n = board.len();
    let m = board[0].len();
    let mut res = vec![];
    for i in 0..n {
        for j in 0..m {
            dfs(i, j, &mut board, &mut res, &mut trie, n, m);
        }
    }
    res.into_iter().collect()
}

fn dfs(
    i: usize,
    j: usize,
    board: &mut Vec<Vec<char>>,
    all: &mut Vec<String>,
    trie: &mut Trie,
    n: usize,
    m: usize,
) {
    let c = board[i][j];
    if let Some(trie) = trie.children.get_mut(&c) {
        board[i][j] = ' ';
        if trie.end.is_some() {
            all.push(trie.end.take().unwrap());
        }
        if i + 1 < n {
            dfs(i + 1, j, board, all, trie, n, m);
        }
        if j + 1 < m {
            dfs(i, j + 1, board, all, trie, n, m);
        }
        if i > 0 {
            dfs(i - 1, j, board, all, trie, n, m);
        }
        if j > 0 {
            dfs(i, j - 1, board, all, trie, n, m);
        }
        board[i][j] = c;
    }
}

#[derive(Default)]
struct Trie {
    children: HashMap<char, Self>,
    end: Option<String>,
}

impl Trie {
    fn insert(&mut self, word: String) {
        let mut link = self;
        for c in word.chars() {
            link = link.children.entry(c).or_default();
        }
        link.end = Some(word);
    }
}
// backtracking trie
#[test]
fn test2_212() {
    assert_eq!(
        find_words(
            vec![
                vec!['o', 'a', 'a', 'n'],
                vec!['e', 't', 'a', 'e'],
                vec!['i', 'h', 'k', 'r'],
                vec!['i', 'f', 'l', 'v']
            ],
            vec![
                String::from("oath"),
                String::from("pea"),
                String::from("eat"),
                String::from("rain")
            ]
        ),
        vec![String::from("oath"), String::from("eat")]
    );
}
