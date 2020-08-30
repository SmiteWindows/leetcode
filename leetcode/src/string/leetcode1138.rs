// https://leetcode-cn.com/problems/alphabet-board-path/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn alphabet_board_path(target: String) -> String {
    let mut pos: Vec<(i32, i32)> = vec![];
    for i in 0..5 {
        for j in 0..5 {
            pos.push((i, j));
        }
    }
    pos.push((5, 0));
    let mut v = vec!['a'];
    for c in target.chars() {
        v.push(c);
    }
    let n = v.len();
    let mut res = "".to_string();
    for i in 1..n {
        let curr = pos[(v[i] as u8 - b'a') as usize];
        let prev = pos[(v[i - 1] as u8 - b'a') as usize];
        let mut r = curr.0 - prev.0;
        let mut c = curr.1 - prev.1;
        while r < 0 {
            res.push('U');
            r += 1;
        }
        while c < 0 {
            res.push('L');
            c += 1;
        }
        while r > 0 {
            res.push('D');
            r -= 1;
        }
        while c > 0 {
            res.push('R');
            c -= 1;
        }
        res.push('!');
    }
    res
}
// hash_table string
#[test]
fn test2_1138() {
    assert_eq!(
        alphabet_board_path(String::from("leet")),
        String::from("DDR!UURRR!!DDD!")
    );
    assert_eq!(
        alphabet_board_path(String::from("code")),
        String::from("RR!DDRR!UUL!R!")
    );
}
