// https://leetcode.com/problems/array-of-doubled-pairs/
// Runtime: 24 ms
// Memory Usage: 2.3 MB
use std::cmp::Ordering::*;
pub fn can_reorder_doubled(a: Vec<i32>) -> bool {
    let mut nonzero = vec![vec![]; 2];
    let mut zero = 0_usize;
    for x in a {
        match x.cmp(&0) {
            Equal => {
                zero += 1;
            }
            Less => {
                nonzero[0].push(-x);
            }
            Greater => {
                nonzero[1].push(x);
            }
        }
    }
    if zero % 2 != 0 || nonzero[0].len() % 2 != 0 || nonzero[1].len() % 2 != 0 {
        return false;
    }
    for i in 0..2 {
        nonzero[i].sort_unstable();
        let size = nonzero[i].len();
        let mut fast = 0;
        for slow in 0..size {
            if nonzero[i][slow] == 0 {
                continue;
            } else {
                while fast < size && nonzero[i][fast] != 2 * nonzero[i][slow] {
                    fast += 1;
                }
                if fast == size {
                    return false;
                } else {
                    nonzero[i][fast] = 0;
                }
            }
        }
    }
    true
}
// hash_table array
#[test]
fn test2_954() {
    assert_eq!(can_reorder_doubled(vec![3, 1, 3, 6]), false);
    assert_eq!(can_reorder_doubled(vec![2, 1, 2, 6]), false);
    assert_eq!(can_reorder_doubled(vec![4, -2, 2, -4]), true);
    assert_eq!(can_reorder_doubled(vec![1, 2, 4, 16, 8, 4]), false);
}
