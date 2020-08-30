// https://leetcode-cn.com/problems/complex-number-multiplication/
#![allow(clippy::many_single_char_names)]
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::{fmt, ops::Mul};
pub fn complex_number_multiply(a: String, b: String) -> String {
    (Complex::from_string(a) * Complex::from_string(b)).to_string()
}

struct Complex {
    r: i32,
    i: i32,
}

impl Complex {
    fn new(r: i32, i: i32) -> Self {
        Self { r, i }
    }

    fn from_string(s: String) -> Self {
        let p = s.find('+').unwrap();
        let n = s.len();
        let r = s[0..p].parse::<i32>().unwrap();
        let i = s[p + 1..n - 1].parse::<i32>().unwrap();
        Self::new(r, i)
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.r * rhs.r - self.i * rhs.i,
            self.r * rhs.i + self.i * rhs.r,
        )
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}+{}i", self.r, self.i)
    }
}
// math string
#[test]
fn test1_537() {
    assert_eq!(
        complex_number_multiply(String::from("1+1i"), String::from("1+1i")),
        String::from("0+2i")
    );
    assert_eq!(
        complex_number_multiply(String::from("1+-1i"), String::from("1+-1i")),
        String::from("0+-2i")
    );
}
