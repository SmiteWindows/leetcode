// https://leetcode-cn.com/problems/parsing-a-boolean-expression/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::{iter::Peekable, str::Chars, vec::IntoIter};
pub fn parse_bool_expr(expression: String) -> bool {
    let tokens = parse_tokens(&mut expression.chars());
    let mut it = tokens.into_iter().peekable();
    parse(&mut it)
}

fn parse(it: &mut Peekable<IntoIter<Tok>>) -> bool {
    let tok = it.next().unwrap();
    match tok {
        Tok::Bool(b) => b,
        Tok::Op('!') => {
            it.next();
            let res = !parse(it);
            it.next();
            res
        }
        Tok::Op('&') => {
            it.next();
            let mut res = parse(it);
            while let Some(&Tok::Op(',')) = it.peek() {
                it.next();
                res &= parse(it);
            }
            it.next();
            res
        }
        Tok::Op('|') => {
            it.next();
            let mut res = parse(it);
            while let Some(&Tok::Op(',')) = it.peek() {
                it.next();
                res |= parse(it);
            }
            it.next();
            res
        }
        _ => panic!(),
    }
}

fn parse_tokens(it: &mut Chars<'_>) -> Vec<Tok> {
    let mut res = vec![];
    for c in it {
        let tok = match c {
            't' => Tok::Bool(true),
            'f' => Tok::Bool(false),
            _ => Tok::Op(c),
        };
        res.push(tok);
    }
    res
}

enum Tok {
    Bool(bool),
    Op(char),
}
// string
#[test]
fn test1_1106() {
    assert_eq!(parse_bool_expr("!(f)".to_string()), true);
    assert_eq!(parse_bool_expr("|(f,t)".to_string()), true);
    assert_eq!(parse_bool_expr("&(t,f)".to_string()), false);
    assert_eq!(parse_bool_expr("|(&(t,f,t),!(t))".to_string()), false);
}
