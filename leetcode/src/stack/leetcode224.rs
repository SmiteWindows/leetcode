// https://leetcode-cn.com/problems/basic-calculator/
// Runtime: 4 ms
// Memory Usage: 4.1 MB
use std::{iter::Peekable, str::Chars, vec::IntoIter};
pub fn calculate(s: String) -> i32 {
    let mut it = s.chars().peekable();
    let tokens = parse_tokens(&mut it);
    let mut it = tokens.into_iter().peekable();
    parse(&mut it)
}

fn parse_term(it: &mut Peekable<IntoIter<Tok>>) -> i32 {
    if eat(it, '(') {
        let res = parse(it);
        eat(it, ')');
        res
    } else if let Some(Tok::Num(x)) = it.next() {
        x
    } else {
        panic!()
    }
}

fn parse(it: &mut Peekable<IntoIter<Tok>>) -> i32 {
    let mut left = parse_term(it);
    loop {
        if eat(it, '+') {
            left += parse_term(it);
            continue;
        }
        if eat(it, '-') {
            left -= parse_term(it);
            continue;
        }
        break;
    }
    left
}

fn eat(it: &mut Peekable<IntoIter<Tok>>, c: char) -> bool {
    if let Some(&Tok::Op(first)) = it.peek() {
        if first == c {
            it.next();
            return true;
        }
    }
    false
}

fn parse_tokens(it: &mut Peekable<Chars<'_>>) -> Vec<Tok> {
    let mut res = vec![];
    while let Some(c) = it.next() {
        match c {
            '0'..='9' => {
                let mut x = (c as u8 - b'0') as i32;
                while let Some('0'..='9') = it.peek() {
                    x *= 10;
                    x += (it.next().unwrap() as u8 - b'0') as i32;
                }
                res.push(Tok::Num(x));
            }
            '(' | ')' | '+' | '-' => {
                res.push(Tok::Op(c));
            }
            _ => {}
        }
    }
    res
}

#[derive(Debug)]
enum Tok {
    Num(i32),
    Op(char),
}
// math stack
#[test]

fn test1_224() {
    assert_eq!(calculate("1 + 1".to_string()), 2);
    assert_eq!(calculate(" 2-1 + 2 ".to_string()), 3);
    assert_eq!(calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
}
