// https://leetcode-cn.com/problems/remove-boxes/
// Runtime: 84 ms
// Memory Usage: 8.6 MB
use std::collections::HashMap;
pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
    let n = boxes.len();
    let mut memo: HashMap<(usize, usize, usize), usize> = HashMap::new();
    dp(0, n, 0, &mut memo, &boxes) as i32
}

fn dp(
    mut start: usize,
    end: usize,
    mut k: usize,
    memo: &mut HashMap<(usize, usize, usize), usize>,
    boxes: &[i32],
) -> usize {
    if start == end {
        return 0;
    }
    if let Some(&res) = memo.get(&(start, end, k)) {
        return res;
    }
    while start + 1 < end && boxes[start + 1] == boxes[start] {
        start += 1;
        k += 1;
    }
    let mut res = dp(start + 1, end, 0, memo, boxes) + (k + 1) * (k + 1);
    for i in start + 1..end {
        if boxes[i] == boxes[start] {
            res = res.max(
                dp(i, end, k + 1, memo, boxes) + dp(start + 1, i, 0, memo, boxes),
            );
        }
    }
    memo.insert((start, end, k), res);
    res
}
// dynamic_programming depth_first_search
#[test]
fn test2_546() {
    assert_eq!(remove_boxes(vec![1, 3, 2, 2, 2, 3, 4, 3, 1]), 23);
}
