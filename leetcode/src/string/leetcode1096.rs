// https://leetcode-cn.com/problems/brace-expansion-ii/
// Runtime: 8 ms
// Memory Usage: 2.3 MB
use std::{
    collections::HashSet,
    iter::{FromIterator, Peekable},
    str::Chars,
    vec::IntoIter,
};
pub fn brace_expansion_ii(expression: String) -> Vec<String> {
    let mut it = expression.chars().peekable();
    let toks = parse_tokens(&mut it);
    let mut it = toks.into_iter().peekable();
    let mut sorted = parse(&mut it).into_iter().collect::<Vec<_>>();
    sorted.sort_unstable();
    sorted
}

fn concat(a: &HashSet<String>, b: &HashSet<String>) -> HashSet<String> {
    let mut res = HashSet::new();
    for i in a {
        for j in b {
            res.insert(format!("{}{}", i, j));
        }
    }
    res
}

fn parse(it: &mut Peekable<IntoIter<Tok>>) -> HashSet<String> {
    let mut res = HashSet::from_iter(vec!["".to_string()]);
    while it.len() != 0 {
        if eat(it, '{') {
            res = concat(&res, &parse(it));
            eat(it, '}');
            continue;
        }
        if eat(it, ',') {
            res = &res | &parse(it);
            continue;
        }
        if let Some(Tok::Op('}')) = it.peek() {
            break;
        }
        if let Some(Tok::S(s)) = it.next() {
            res = concat(&res, &HashSet::from_iter(vec![s]));
            continue;
        }
    }
    res
}

fn eat(it: &mut Peekable<IntoIter<Tok>>, c: char) -> bool {
    if let Some(&Tok::Op(t)) = it.peek() {
        if t == c {
            it.next();
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn parse_tokens(it: &mut Peekable<Chars<'_>>) -> Vec<Tok> {
    let mut res = Vec::new();
    while let Some(c) = it.next() {
        match c {
            '{' | '}' | ',' => res.push(Tok::Op(c)),
            'a'..='z' => {
                let mut s = "".to_string();
                s.push(c);
                while let Some('a'..='z') = it.peek() {
                    s.push(it.next().unwrap());
                }
                res.push(Tok::S(s));
            }
            _ => {}
        }
    }
    res
}

#[derive(Debug)]
enum Tok {
    S(String),
    Op(char),
}
// string
#[test]
fn test1_1096() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        brace_expansion_ii("{a,b}{c,{d,e}}".to_string()),
        vec_string!["ac", "ad", "ae", "bc", "bd", "be"]
    );
    assert_eq!(
        brace_expansion_ii("{{a,z},a{b,c},{ab,z}}".to_string()),
        vec_string!["a", "ab", "ac", "z"]
    );
}
