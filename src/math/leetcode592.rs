// https://leetcode.com/problems/fraction-addition-and-subtraction/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::fmt::{Display, Formatter, Result};
pub fn fraction_addition(expression: String) -> String {
    let mut toks = vec![];
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
                        val += (c_it.next().expect("exist") as u8 - b'0') as i32;
                    } else {
                        break;
                    }
                }
                toks.push(Tok::Num(val));
            }
        }
    }
    let mut t_it = toks.into_iter().peekable();
    let mut fractions = vec![];
    while t_it.peek().is_some() {
        let mut sign = 1;
        let numerator;
        let denominator;
        if let Some(Tok::Op('-')) = t_it.peek() {
            t_it.next().expect("exist");
            sign = -1;
        }
        if let Some(Tok::Op('+')) = t_it.peek() {
            t_it.next().expect("exist");
        }
        if let Tok::Num(x) = t_it.next().expect("exist") {
            numerator = x;
        } else {
            panic!();
        }
        if let Tok::Op('/') = t_it.next().expect("exist") {
        } else {
            panic!();
        }
        if let Tok::Num(y) = t_it.next().expect("exist") {
            denominator = y;
        } else {
            panic!();
        }
        fractions.push(Fraction::new(sign, numerator, denominator));
    }
    while fractions.len() > 1 {
        let a = fractions.pop().expect("exist");
        let b = fractions.pop().expect("exist");
        let c = a.add(b);
        fractions.push(c);
    }
    let mut res = fractions.pop().expect("exist");
    res = res.reduce();
    res.to_string()
}

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
        Self {
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
        Self {
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
        Self {
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
