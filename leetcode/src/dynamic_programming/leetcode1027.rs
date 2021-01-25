// https://leetcode-cn.com/problems/longest-arithmetic-sequence/
// Runtime: 616 ms
// Memory Usage: 9.8 MB
use std::collections::HashMap;
pub fn longest_arith_seq_length(a: Vec<i32>) -> i32 {
    dp(&a) as i32
}
fn dp(a: &[i32]) -> usize {
    if a.len() <= 1 {
        return 0;
    }
    // "2" numbers is because at this point there's always at least a sequence of two numbers
    let mut m = vec![vec![2; a.len()]; a.len()];
    let mut max_seq = 2;
    let mut seen_numbers = HashMap::<i32, Vec<usize>>::new(); // number: [indices]
    for right_edge in 0..a.len() {
        for i in 0..right_edge {
            let diff = a[right_edge] - a[i];
            let prev_in_sequence = a[i] - diff;
            if seen_numbers.contains_key(&prev_in_sequence) {
                for number_index in seen_numbers.get(&prev_in_sequence).unwrap().iter().rev() {
                    if *number_index < i {
                        m[i][right_edge] = m[i][right_edge].max(m[*number_index][i] + 1);
                        break;
                    }
                }
            }
            if m[i][right_edge] > max_seq {
                max_seq = m[i][right_edge];
            }
        }
        if let Some(val) = seen_numbers.get_mut(&a[right_edge]) {
            val.push(right_edge);
        } else {
            seen_numbers.insert(a[right_edge], vec![right_edge]);
        }
    }
    max_seq
}
// dynamic_programming
#[test]
fn test1_1027() {
    assert_eq!(longest_arith_seq_length(vec![3, 6, 9, 12]), 4);
    assert_eq!(longest_arith_seq_length(vec![9, 4, 7, 2, 10]), 3);
    assert_eq!(longest_arith_seq_length(vec![20, 1, 15, 3, 10, 5, 8]), 4);
}
