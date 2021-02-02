// https://leetcode-cn.com/problems/open-the-lock/
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
    let mut res = Vec::new();
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
    use leetcode_prelude::vec_string;
    assert_eq!(
        open_lock(
            vec_string!["0201", "0101", "0102", "1212", "2002"],
            "0202".to_string()
        ),
        6
    );
    assert_eq!(open_lock(vec_string!["8888"], "0009".to_string()), 1);
    assert_eq!(
        open_lock(
            vec_string!["8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"],
            "8888".to_string()
        ),
        -1
    );
    assert_eq!(open_lock(vec_string!["0000"], "8888".to_string()), -1);
}
