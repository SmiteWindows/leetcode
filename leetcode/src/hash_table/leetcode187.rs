// https://leetcode-cn.com/problems/repeated-dna-sequences/
// Runtime: 8 ms
// Memory Usage: 3.6 MB
use std::collections::HashSet;
pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    let n = s.len();
    let mut hash = 0;
    let mut once = HashSet::new();
    let mut twice = HashSet::new();
    let s = s.chars().collect::<Vec<_>>();
    for i in (0..n).rev() {
        hash <<= 2;
        hash |= val(s[i]);
        if i + 10 < n {
            hash -= val(s[i + 10]) << 20;
        }
        if i + 10 <= n && !once.insert(hash) {
            twice.insert(hash);
        }
    }
    twice.into_iter().map(decode).collect()
}

fn val(c: char) -> u32 {
    match c {
        'A' => 0,
        'C' => 1,
        'G' => 2,
        _ => 3,
    }
}

fn decode(mut hash: u32) -> String {
    let mut res = "".to_string();
    for _ in 0..10 {
        res.push(match hash & 3 {
            0 => 'A',
            1 => 'C',
            2 => 'G',
            _ => 'T',
        });
        hash >>= 2;
    }
    res
}
// bit_manipulation hash_table
#[test]
fn test2_187() {
    use leetcode_prelude::{assert_eq_sorted, vec_string};
    assert_eq_sorted!(
        find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string()),
        vec_string!["AAAAACCCCC", "CCCCCAAAAA"]
    );
}
