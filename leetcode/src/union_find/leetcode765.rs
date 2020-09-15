// https://leetcode-cn.com/problems/couples-holding-hands/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn min_swaps_couples(mut row: Vec<i32>) -> i32 {
    let n = row.len();
    let mut res = 0;
    for i in 0..n {
        if i % 2 == 1 {
            continue;
        }
        if row[i] == row[i + 1] ^ 1 {
            continue;
        }
        res += 1;
        for j in i + 2..n {
            if row[i] == row[j] ^ 1 {
                row.swap(i + 1, j);
                break;
            }
        }
    }
    res
}
// graph greedy union_find
#[test]
fn test1_765() {
    assert_eq!(min_swaps_couples(vec![0, 2, 1, 3]), 1);
    assert_eq!(min_swaps_couples(vec![3, 2, 0, 1]), 0);
}
