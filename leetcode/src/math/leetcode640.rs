// https://leetcode-cn.com/problems/solve-the-equation/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn solve_equation(equation: String) -> String {
    let mut it = equation.split('=');
    let left = it.next().unwrap();
    let right = it.next().unwrap();
    let (a, b) = parse(left);
    let (c, d) = parse(right);
    if a == c {
        if b == d {
            "Infinite solutions".to_string()
        } else {
            "No solution".to_string()
        }
    } else {
        format!("x={}", (d - b) / (a - c))
    }
}

fn parse(s: &str) -> (i32, i32) {
    let mut sign = 1;
    let mut x = false;
    let mut val = None;
    let mut a = 0;
    let mut b = 0;
    for c in s.chars() {
        match c {
            'x' => {
                if val.is_none() {
                    val = Some(1);
                }
                x = true;
            }
            '+' => {
                if let Some(v) = val {
                    if x {
                        a += sign * v;
                    } else {
                        b += sign * v;
                    }
                }
                val = None;
                x = false;
                sign = 1;
            }
            '-' => {
                if let Some(v) = val {
                    if x {
                        a += sign * v;
                    } else {
                        b += sign * v;
                    }
                }
                val = None;
                x = false;
                sign = -1;
            }
            _ => {
                val = if let Some(mut v) = val {
                    v *= 10;
                    v += (c as u8 - b'0') as i32;
                    Some(v)
                } else {
                    Some((c as u8 - b'0') as i32)
                };
            }
        }
    }
    if x {
        if val.is_none() {
            val = Some(1);
        }
        a += sign * val.unwrap();
    } else {
        b += sign * val.unwrap();
    }
    (a, b)
}
// math
#[test]
fn test1_640() {
    assert_eq!(
        solve_equation(String::from("x+5-3+x=6+x-2")),
        String::from("x=2")
    );
    assert_eq!(
        solve_equation(String::from("x=x")),
        String::from("Infinite solutions")
    );
    assert_eq!(solve_equation(String::from("2x=x")), String::from("x=0"));
    assert_eq!(
        solve_equation(String::from("2x+3x-6x=x+2")),
        String::from("x=-1")
    );
    assert_eq!(
        solve_equation(String::from("x=x+2")),
        String::from("No solution")
    );
}
