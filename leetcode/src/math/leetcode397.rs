// https://leetcode-cn.com/problems/integer-replacement/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn integer_replacement(n: i32) -> i32 {
    dfs(n as u32)
}

fn dfs(mut n: u32) -> i32 {
    if n == 1 {
        0
    } else {
        let mut zeros = 0;
        while n & 1 == 0 {
            n >>= 1;
            zeros += 1;
        }
        if n == 1 {
            zeros
        } else {
            zeros + 1 + dfs(n + 1).min(dfs(n - 1))
        }
    }
}
// math bit_manipulation
#[test]
fn test1_397() {
    assert_eq!(integer_replacement(8), 3);
    assert_eq!(integer_replacement(7), 4);
}
