// https://leetcode-cn.com/problems/array-of-doubled-pairs/
// Runtime: 24 ms
// Memory Usage: 2.3 MB
use std::cmp::Ordering;
pub fn can_reorder_doubled(a: Vec<i32>) -> bool {
    let mut nonzero = vec![Vec::new(); 2];
    let mut zero = 0_usize;
    for x in a {
        match x.cmp(&0) {
            Ordering::Equal => {
                zero += 1;
            }
            Ordering::Less => {
                nonzero[0].push(-x);
            }
            Ordering::Greater => {
                nonzero[1].push(x);
            }
        }
    }
    if zero % 2 != 0 || nonzero[0].len() % 2 != 0 || nonzero[1].len() % 2 != 0 {
        return false;
    }
    for nz in nonzero.iter_mut().take(2) {
        nz.sort_unstable();
        let size = nz.len();
        let mut fast = 0;
        for slow in 0..size {
            if nz[slow] == 0 {
                continue;
            } else {
                while fast < size && nz[fast] != 2 * nz[slow] {
                    fast += 1;
                }
                if fast == size {
                    return false;
                } else {
                    nz[fast] = 0;
                }
            }
        }
    }
    true
}
// hash_table array
#[test]
fn test1_954() {
    assert_eq!(can_reorder_doubled(vec![3, 1, 3, 6]), false);
    assert_eq!(can_reorder_doubled(vec![2, 1, 2, 6]), false);
    assert_eq!(can_reorder_doubled(vec![4, -2, 2, -4]), true);
    assert_eq!(can_reorder_doubled(vec![1, 2, 4, 16, 8, 4]), false);
}
