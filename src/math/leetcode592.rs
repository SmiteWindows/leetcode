// https://leetcode.com/problems/fraction-addition-and-subtraction/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::fmt::{Display, Formatter, Result};

enum Tok {
    Op(char),
    Num(i32),
}

struct Fraction {
    sign: i32,
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    fn new(sign: i32, numerator: i32, denominator: i32) -> Self {
        Fraction {
            sign,
            numerator,
            denominator,
        }
    }

    fn add(self, other: Self) -> Self {
        let mut numerator = self.sign * self.numerator * other.denominator
            + other.sign * other.numerator * self.denominator;
        let denominator = self.denominator * other.denominator;
        let mut sign = 1;
        if numerator < 0 {
            sign *= -1;
            numerator *= -1;
        }
        Fraction {
            sign,
            numerator,
            denominator,
        }
    }
    
    fn reduce(self) -> Self {
        let sign = self.sign;
        let mut denominator = self.denominator;
        let mut numerator = self.numerator;
        let gcd = gcd(denominator, numerator);
        denominator /= gcd;
        numerator /= gcd;
        Fraction {
            sign,
            numerator,
            denominator,
        }
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.sign > 0 {
            write!(f, "{}/{}", self.numerator, self.denominator)
        } else {
            write!(f, "-{}/{}", self.numerator, self.denominator)
        }
    }
}

fn gcd(mut m: i32, mut n: i32) -> i32 {
    while m != 0 {
        let temp = m;
        m = n % temp;
        n = temp;
    }
    n.abs()
}
pub fn fraction_addition(expression: String) -> String {
    let mut toks: Vec<Tok> = vec![];
    let mut c_it = expression.chars().peekable();
    while let Some(c) = c_it.next() {
        match c {
            '-' | '+' | '/' => {
                toks.push(Tok::Op(c));
            }
            _ => {
                let mut val = (c as u8 - b'0') as i32;
                while let Some(p) = c_it.peek() {
                    if p.is_digit(10) {
                        val *= 10;
                        val += (c_it.next().unwrap() as u8 - b'0') as i32;
                    } else {
                        break;
                    }
                }
                toks.push(Tok::Num(val));
            }
        }
    }
    let mut t_it = toks.into_iter().peekable();
    let mut fractions: Vec<Fraction> = vec![];
    while t_it.peek().is_some() {
        let mut sign = 1;
        let numerator;
        let denominator;
        if let Some(Tok::Op('-')) = t_it.peek() {
            t_it.next().unwrap();
            sign = -1;
        }
        if let Some(Tok::Op('+')) = t_it.peek() {
            t_it.next().unwrap();
        }
        if let Tok::Num(x) = t_it.next().unwrap() {
            numerator = x;
        } else {
            panic!();
        }
        if let Tok::Op('/') = t_it.next().unwrap() {
        } else {
            panic!();
        }
        if let Tok::Num(y) = t_it.next().unwrap() {
            denominator = y;
        } else {
            panic!();
        }
        fractions.push(Fraction::new(sign, numerator, denominator));
    }
    while fractions.len() > 1 {
        let a = fractions.pop().unwrap();
        let b = fractions.pop().unwrap();
        let c = a.add(b);
        fractions.push(c);
    }
    let mut res = fractions.pop().unwrap();
    res = res.reduce();
    res.to_string()
}
// math
#[test]
fn test1_592() {
    assert_eq!(
        fraction_addition(String::from("-1/2+1/2")),
        String::from("0/1")
    );
    assert_eq!(
        fraction_addition(String::from("-1/2+1/2+1/3")),
        String::from("1/3")
    );
    assert_eq!(
        fraction_addition(String::from("1/3-1/2")),
        String::from("-1/6")
    );
    assert_eq!(
        fraction_addition(String::from("5/3+1/3")),
        String::from("2/1")
    );
}
