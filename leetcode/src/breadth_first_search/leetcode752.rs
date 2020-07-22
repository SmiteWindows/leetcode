// https://leetcode.com/problems/open-the-lock/
// Runtime: 12 ms
// Memory Usage: 2.1 MB
use std::collections::HashSet;
pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let mut visited = deadends.into_iter().map(s2x).collect::<HashSet<u32>>();
    let target = s2x(target);
    let start = 0;
    if visited.contains(&start) {
        return -1;
    }
    let mut begin = HashSet::new();
    let mut end = HashSet::new();
    begin.insert(start);
    end.insert(target);
    let mut level = 0;
    while !begin.is_empty() && !end.is_empty() {
        let mut temp = HashSet::new();
        for x in begin {
            if end.contains(&x) {
                return level;
            } else {
                visited.insert(x);
                for y in adj(x) {
                    if !visited.contains(&y) {
                        temp.insert(y);
                    }
                }
            }
        }
        level += 1;
        begin = end;
        end = temp
    }
    -1
}

fn s2x(s: String) -> u32 {
    let mut res = 0;
    for (i, b) in s.bytes().map(|b| (b - b'0') as u32).enumerate() {
        res |= b << (i * 8);
    }
    res
}

fn x2s(x: u32) -> String {
    let mut v = vec![0_u8; 4];
    for (i, vi) in v.iter_mut().enumerate().take(4) {
        *vi = ((x >> (i * 8)) & 0xff) as u8;
    }
    v.into_iter().map(|b| (b + b'0') as char).collect()
}

fn adj(x: u32) -> Vec<u32> {
    let mut res = vec![];
    for i in 0..4 {
        let b1 = (((x >> (i * 8) & 0xff) + 1) % 10) << (i * 8);
        let b2 = (((x >> (i * 8) & 0xff) + 9) % 10) << (i * 8);
        let mask = !(0xff << (i * 8));
        res.push((x & mask) | b1);
        res.push((x & mask) | b2);
    }
    res
}
// breadth_first_search
#[test]
fn test1_752() {
    assert_eq!(
        open_lock(
            vec![
                String::from("0201"),
                String::from("0101"),
                String::from("0102"),
                String::from("1212"),
                String::from("2002")
            ],
            String::from("0202")
        ),
        6
    );
    assert_eq!(
        open_lock(vec![String::from("8888")], String::from("0009")),
        1
    );
    assert_eq!(
        open_lock(
            vec![
                String::from("8887"),
                String::from("8889"),
                String::from("8878"),
                String::from("8898"),
                String::from("8788"),
                String::from("8988"),
                String::from("7888"),
                String::from("9888")
            ],
            String::from("8888")
        ),
        -1
    );
    assert_eq!(
        open_lock(vec![String::from("0000")], String::from("8888")),
        -1
    );
}
