// https://leetcode-cn.com/problems/cinema-seat-allocation/
// Runtime: 36 ms
// Memory Usage: 2.9 MB
use std::collections::HashMap;
pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut hm: HashMap<usize, u16> = HashMap::new();
    for seat in reserved_seats {
        let i = (seat[0] - 1) as usize;
        let j = (seat[1] - 1) as usize;
        *hm.entry(i).or_default() |= 1 << j;
    }
    let mut res = 0;
    for &row_bitset in hm.values() {
        res += num_of_families(row_bitset);
    }
    res += (n - hm.len()) * 2;
    res as i32
}

fn num_of_families(row_bitset: u16) -> usize {
    let two = 0b0111111110;
    let mid = 0b0001111000;
    let left = 0b0111100000;
    let right = 0b0000011110;
    if row_bitset & two == 0 {
        2
    } else if row_bitset & mid == 0 || row_bitset & left == 0 || row_bitset & right == 0 {
        1
    } else {
        0
    }
}
// greedy array
#[test]
fn test2_1386() {
    use leetcode_prelude::vec2;
    assert_eq!(
        max_number_of_families(3, vec2![[1, 2], [1, 3], [1, 8], [2, 6], [3, 1], [3, 10]]),
        4
    );
    assert_eq!(max_number_of_families(2, vec2![[2, 1], [1, 8], [2, 6]]), 2);
    assert_eq!(
        max_number_of_families(4, vec2![[4, 3], [1, 4], [4, 6], [1, 7]]),
        4
    );
}
