// https://leetcode.com/problems/prison-cells-after-n-days/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::collections::HashMap;
pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
    // let mut cells = cells;
    // let mut n = n;
    // let mut hm: HashMap<Vec<i32>, i32> = HashMap::new();
    // while n > 0 {
    //     hm.insert(cells.to_vec(), n);
    //     let mut next = vec![0; 8];
    //     for i in 1..7 {
    //         next[i] = 1 - (cells[i - 1] ^ cells[i + 1]);
    //     }
    //     cells = next;
    //     n -= 1;
    //     if let Some(m) = hm.get(&cells) {
    //         n %= m - n;
    //     }
    // }
    // cells
    let mut n = n;
    let mut fast_forwarded = false;
    let mut seen = HashMap::new();
    let mut state_bitmap = 0x0;
    for cell in cells.iter() {
        state_bitmap <<= 1;
        state_bitmap |= cell;
    }
    while n > 0 {
        if !fast_forwarded {
            if seen.contains_key(&state_bitmap) {
                n %= seen[&state_bitmap] - n;
                fast_forwarded = true;
            } else {
                seen.insert(state_bitmap, n);
            }
            // seen.entry(state_bitmap).or_insert(n);
        }
        if n > 0 {
            state_bitmap = !(state_bitmap << 1) ^ (state_bitmap >> 1);
            state_bitmap &= 0x7e;
            n -= 1;
        }
    }
    let mut output = vec![];
    for _ in 0..cells.len() {
        output.push(state_bitmap & 1);
        state_bitmap >>= 1;
    }
    output.into_iter().rev().collect()
}
// hash_table
#[test]
fn test1_957() {
    assert_eq!(
        prison_after_n_days(vec![0, 1, 0, 1, 1, 0, 0, 1], 7),
        vec![0, 0, 1, 1, 0, 0, 0, 0]
    );
    assert_eq!(
        prison_after_n_days(vec![1, 0, 0, 1, 0, 0, 1, 0], 1000000000),
        vec![0, 0, 1, 1, 1, 1, 1, 0]
    );
}
