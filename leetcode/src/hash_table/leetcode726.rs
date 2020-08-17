// https://leetcode.com/problems/number-of-atoms/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{collections::BTreeMap, iter::Peekable, str::Chars, vec::IntoIter};
pub fn count_of_atoms(formula: String) -> String {
    let mut it = formula.chars().peekable();
    let tokens = parse_tokens(&mut it);
    let count: BTreeMap<String, usize> = parse(&mut tokens.into_iter().peekable());
    count
        .into_iter()
        .map(|(k, v)| if v > 1 { format!("{}{}", k, v) } else { k })
        .collect()
}

fn parse(it: &mut Peekable<IntoIter<Tok>>) -> BTreeMap<String, usize> {
    let mut res: BTreeMap<String, usize> = BTreeMap::new();
    loop {
        match it.peek() {
            Some(Tok::Name(_)) => {
                if let Some(Tok::Name(s)) = it.next() {
                    let x = if let Some(&Tok::Num(x)) = it.peek() {
                        it.next();
                        x
                    } else {
                        1
                    };
                    *res.entry(s).or_default() += x;
                }
            }
            Some(&Tok::Op('(')) => {
                it.next();
                let inside = parse(it);
                it.next();
                let x = if let Some(&Tok::Num(x)) = it.peek() {
                    it.next();
                    x
                } else {
                    1
                };
                for (k, v) in inside {
                    *res.entry(k).or_default() += v * x;
                }
            }
            Some(&Tok::Op(')')) | None => {
                break;
            }
            _ => panic!(),
        }
    }
    res
}

fn parse_tokens(it: &mut Peekable<Chars<'_>>) -> Vec<Tok> {
    let mut res = vec![];
    while let Some(c) = it.next() {
        match c {
            '(' | ')' => res.push(Tok::Op(c)),
            '0'..='9' => {
                let mut x = (c as u8 - b'0') as usize;
                while let Some('0'..='9') = it.peek() {
                    x *= 10;
                    let y = (it.next().unwrap() as u8 - b'0') as usize;
                    x += y;
                }
                res.push(Tok::Num(x));
            }
            'A'..='Z' => {
                let mut s = "".to_string();
                s.push(c);
                while let Some('a'..='z') = it.peek() {
                    s.push(it.next().unwrap());
                }
                res.push(Tok::Name(s));
            }
            _ => panic!(),
        }
    }
    res
}

enum Tok {
    Num(usize),
    Name(String),
    Op(char),
}

// hash_table stack recursion
#[test]
fn test2_726() {
    assert_eq!(count_of_atoms(String::from("H2O")), String::from("H2O"));
    assert_eq!(
        count_of_atoms(String::from("Mg(OH)2")),
        String::from("H2MgO2")
    );
    assert_eq!(
        count_of_atoms(String::from("K4(ON(SO3)2)2")),
        String::from("K4N2O14S4")
    );
}
