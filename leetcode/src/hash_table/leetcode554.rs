// https://leetcode-cn.com/problems/brick-wall/
// Runtime: 12 ms
// Memory Usage: 2.8 MB
use std::collections::HashMap;
pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
    let n = wall.len();
    let w = wall[0].iter().sum::<i32>();
    let mut hm: HashMap<i32, usize> = HashMap::new();
    for row in wall {
        let mut sum = 0;
        for x in row {
            sum += x;
            *hm.entry(sum).or_default() += 1;
        }
    }
    let mut max = 0;
    for (k, v) in hm {
        if k != w {
            max = max.max(v);
        }
    }
    (n - max) as i32
}
// hash_table
#[test]
fn test1_554() {
    use leetcode_prelude::vec2;
    assert_eq!(
        least_bricks(vec2![
            [1, 2, 2, 1],
            [3, 1, 2],
            [1, 3, 2],
            [2, 4],
            [3, 1, 2],
            [1, 3, 1, 1]
        ]),
        2
    );
}
