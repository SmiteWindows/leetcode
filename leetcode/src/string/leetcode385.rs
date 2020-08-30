// https://leetcode-cn.com/problems/mini-parser/
// Runtime: 0 ms
// Memory Usage: 2.6 MB
pub fn deserialize(s: String) -> NestedInteger {
    NestedIntegerParser::new(s).parse().unwrap()
}

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

use std::{iter::Peekable, vec::IntoIter};
use NestedInteger::*;
use Tok::*;
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tok {
    Num(i32),
    Op(char),
}

pub struct NestedIntegerParser {
    it: Peekable<IntoIter<Tok>>,
}

impl NestedIntegerParser {
    pub fn new<T>(s: T) -> Self
    where
        T: Into<String>,
    {
        let s = s.into();
        let mut tokens = vec![];
        let mut it = s.chars().peekable();
        while let Some(c) = it.next() {
            match c {
                '0'..='9' => {
                    let mut val = (c as u8 - b'0') as i32;
                    while let Some(&c) = it.peek() {
                        match c {
                            '0'..='9' => {
                                it.next();
                                val *= 10;
                                val += (c as u8 - b'0') as i32;
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    tokens.push(Num(val));
                }
                _ => {
                    tokens.push(Op(c));
                }
            }
        }
        Self {
            it: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Option<NestedInteger> {
        if !self.eat('[') {
            return self.parse_int();
        }
        let mut list = vec![];
        while let Some(x) = self.parse() {
            list.push(x);
            self.eat(',');
        }
        if !self.eat(']') {
            return None;
        }
        Some(List(list))
    }

    fn parse_int(&mut self) -> Option<NestedInteger> {
        let sign = if self.eat('-') { -1 } else { 1 };
        if let Some(&Tok::Num(num)) = self.it.peek() {
            self.it.next();
            return Some(Int(sign * num));
        }
        None
    }

    fn eat(&mut self, c: char) -> bool {
        if let Some(&Tok::Op(t)) = self.it.peek() {
            if t == c {
                self.it.next();
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
// stack string
#[test]
fn test2_385() {
    use self::NestedInteger::*;
    assert_eq!(deserialize(String::from("324")), Int(324));
    assert_eq!(
        deserialize(String::from("[123,[456,[789]]]")),
        List(vec![Int(123), List(vec![Int(456), List(vec![Int(789)])])])
    );
}
