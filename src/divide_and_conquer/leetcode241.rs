// https://leetcode.com/problems/different-ways-to-add-parentheses/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
    let mut val = 0;
    let mut toks: Vec<Tok> = vec![];
    for c in input.chars() {
        match c {
            '+' | '-' | '*' => {
                toks.push(Tok::Val(val));
                toks.push(Tok::Op(c));
                val = 0;
            }
            _ => {
                val *= 10;
                val += (c as u8 - b'0') as i32;
            }
        }
    }
    toks.push(Tok::Val(val));
    eval(&toks)
}

fn eval(toks: &[Tok]) -> Vec<i32> {
    let mut res = vec![];
    let n = toks.len();
    for i in 0..n {
        if let Tok::Op(c) = toks[i] {
            let left = eval(&toks[0..i]);
            let right = eval(&toks[i + 1..n]);
            for l in &left {
                for r in &right {
                    match c {
                        '+' => {
                            res.push(l + r);
                        }
                        '-' => {
                            res.push(l - r);
                        }
                        '*' => {
                            res.push(l * r);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    if res.is_empty() {
        if let Tok::Val(val) = toks[0] {
            vec![val]
        } else {
            unreachable!()
        }
    } else {
        res
    }
}

enum Tok {
    Op(char),
    Val(i32),
}
// divide_and_conquer
#[test]
fn test1_241() {
    let input = "2-1-1".to_string();
    let mut res = vec![0, 2];
    let mut ans = diff_ways_to_compute(input);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let input = "2*3-4*5".to_string();
    let mut res = vec![-34, -14, -10, -10, 10];
    let mut ans = diff_ways_to_compute(input);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
