// https://leetcode.com/problems/repeated-dna-sequences/
use std::collections::{HashMap, HashSet};
pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    let l = 10;
    let n = s.len();
    if n <= l {
        return vec![];
    }
    let a: usize = 4;
    let al = a.pow(l as u32);
    let mut to_int = HashMap::new();
    to_int.insert('A', 0);
    to_int.insert('C', 1);
    to_int.insert('G', 2);
    to_int.insert('T', 3);
    let mut nums = vec![0; n];
    for i in 0..n {
        nums[i] = to_int[&s.chars().nth(i).unwrap()];
    }
    let mut bitmask = 0;
    let mut seen = HashSet::new();
    let mut output = HashSet::new();
    for start in 0..n - l + 1 {
        if start != 0 {
            bitmask <<= 2;
            bitmask |= nums[start + l - 1];
            bitmask &= !(3 << (2 * l));
        } else {
            for i in nums.iter().take(l) {
                bitmask <<= 2;
                bitmask |= i;
            }
        }
        if seen.contains(&bitmask) {
            output.insert(s[start..start + l].to_string());
        }
        seen.insert(bitmask);
    }
    output.into_iter().collect()
}
// bit_manipulation hash_table
#[test]
fn test1_187() {
    let mut res = find_repeated_dna_sequences(String::from("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"));
    res.sort();
    assert_eq!(
        res,
        vec![String::from("AAAAACCCCC"), String::from("CCCCCAAAAA")]
    );
}
