// https://leetcode-cn.com/problems/count-unhappy-friends/
// Runtime: 16 ms
// Memory Usage: 4.9 MB
use std::collections::HashSet;
pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut list = vec![HashSet::new(); n];
    for pair in pairs {
        let x = pair[0] as usize;
        let y = pair[1] as usize;
        for &u in &preferences[x] {
            let u = u as usize;
            if u != y {
                list[x].insert(u);
            } else {
                break;
            }
        }
        for &v in &preferences[y] {
            let v = v as usize;
            if v != x {
                list[y].insert(v);
            } else {
                break;
            }
        }
    }
    let mut res = 0;
    for i in 0..n {
        for j in 0..n {
            if i != j && list[i].contains(&j) && list[j].contains(&i) {
                res += 1;
                break;
            }
        }
    }
    res
}
// array
#[test]
fn test1_1583() {
    use leetcode_prelude::vec2;
    assert_eq!(
        unhappy_friends(
            4,
            vec2![[1, 2, 3], [3, 2, 0], [3, 1, 0], [1, 2, 0]],
            vec2![[0, 1], [2, 3]]
        ),
        2
    );
    assert_eq!(unhappy_friends(2, vec2![[1], [0]], vec2![[1, 0]]), 0);
    assert_eq!(
        unhappy_friends(
            4,
            vec2![[1, 3, 2], [2, 3, 0], [1, 3, 0], [0, 2, 1]],
            vec2![[1, 3], [0, 2]]
        ),
        4
    );
}
