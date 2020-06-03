// https://leetcode.com/problems/rotated-digits/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn rotated_digits(n: i32) -> i32 {
    let n = n as usize;
    let mut a = vec![D::Invalid; (n + 1) as usize];
    for i in 0..=n {
        if i < 10 {
            a[i] = D::new(i);
        } else {
            let j = i / 10;
            let k = i % 10;
            a[i] = match (a[j], a[k]) {
                (D::Invalid, _) | (_, D::Invalid) => D::Invalid,
                (D::Different, _) | (_, D::Different) => D::Different,
                (D::Same, D::Same) => D::Same,
            }
        }
    }
    a.iter()
        .fold(0, |sum, &d| if d == D::Different { sum + 1 } else { sum })
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum D {
    Invalid,
    Same,
    Different,
}

impl D {
    fn new(d: usize) -> Self {
        match d {
            0 | 1 | 8 => D::Same,
            2 | 5 | 6 | 9 => D::Different,
            _ => D::Invalid,
        }
    }
}
// string
#[test]
fn test1_788() {
    assert_eq!(rotated_digits(10), 4);
}
