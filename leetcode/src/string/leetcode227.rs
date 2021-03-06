// https://leetcode-cn.com/problems/basic-calculator-ii/
// Runtime: 0 ms
// Memory Usage: 4 MB
use std::{iter::Peekable, slice::Iter};
use Tok::*;
pub fn calculate(s: String) -> i32 {
    let tokens = tokens(&s);
    let mut it = tokens.iter().peekable();
    if let Some(val) = parse_expr(&mut it) {
        val
    } else {
        0
    }
}

fn parse_expr(it: &mut Peekable<Iter<'_, Tok>>) -> Option<i32> {
    let mut lhs = parse_factor(it)?;
    while let Some(&tok) = it.peek() {
        if tok.is_expr_op() {
            let op = *it.next().unwrap();
            let rhs = parse_factor(it)?;
            lhs = op.eval(lhs, rhs)?;
        } else {
            break;
        }
    }
    Some(lhs)
}

fn parse_factor(it: &mut Peekable<Iter<'_, Tok>>) -> Option<i32> {
    let mut lhs = parse_num(it)?;
    while let Some(&tok) = it.peek() {
        if tok.is_factor_op() {
            let op = *it.next().unwrap();
            let rhs = parse_num(it)?;
            lhs = op.eval(lhs, rhs)?;
        } else {
            break;
        }
    }
    Some(lhs)
}

fn parse_num(it: &mut Peekable<Iter<'_, Tok>>) -> Option<i32> {
    match it.next() {
        Some(Tok::Num(x)) => Some(*x),
        _ => None,
    }
}

fn tokens(s: &str) -> Vec<Tok> {
    let mut v = Vec::new();
    let mut it = s.chars().peekable();
    while let Some(c) = it.next() {
        match c {
            '0'..='9' => {
                let mut x = (c as u8 - b'0') as i32;
                while let Some(c) = it.peek() {
                    if c.is_numeric() {
                        x *= 10;
                        x += (it.next().unwrap() as u8 - b'0') as i32;
                    } else {
                        break;
                    }
                }
                v.push(Tok::Num(x));
            }
            '+' | '-' | '*' | '/' => {
                v.push(Tok::Op(c));
            }
            _ => {}
        }
    }
    v
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tok {
    Num(i32),
    Op(char),
}

impl Tok {
    fn is_expr_op(self) -> bool {
        matches!(self, Op('+') | Op('-'))
    }

    fn is_factor_op(self) -> bool {
        matches!(self, Op('*') | Op('/'))
    }

    fn val(self) -> Option<i32> {
        match self {
            Num(x) => Some(x),
            _ => None,
        }
    }

    fn eval(self, lhs: i32, rhs: i32) -> Option<i32> {
        match self {
            Op('+') => Some(lhs + rhs),
            Op('-') => Some(lhs - rhs),
            Op('*') => Some(lhs * rhs),
            Op('/') => {
                if rhs != 0 {
                    Some(lhs / rhs)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
// string
#[test]
fn test1_227() {
    assert_eq!(calculate("3+2*2".to_string()), 7);
    assert_eq!(calculate(" 3/2 ".to_string()), 1);
    assert_eq!(calculate(" 3+5 / 2 ".to_string()), 5);
}
