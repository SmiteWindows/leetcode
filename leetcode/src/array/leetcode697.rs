// https://leetcode-cn.com/problems/degree-of-an-array/
// Runtime: 4 ms
// Memory Usage: 2.6 MB
use std::collections::HashMap;
pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    let mut hm = HashMap::new();
    let mut max_degree = 0;
    for (i, &x) in nums.iter().enumerate() {
        let e = hm.entry(x).or_insert(Degree {
            left: i,
            right: i,
            count: 0,
        });
        e.left = e.left.min(i);
        e.right = e.right.max(i);
        e.count += 1;
        max_degree = e.count.max(max_degree);
    }
    let mut min_width = nums.len();
    for d in hm.values() {
        if d.count == max_degree {
            min_width = min_width.min(d.right - d.left + 1);
        }
    }
    min_width as i32
}

struct Degree {
    left: usize,
    right: usize,
    count: usize,
}
// array
#[test]
fn test1_697() {
    assert_eq!(find_shortest_sub_array(vec![1, 2, 2, 3, 1]), 2);
    assert_eq!(find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2]), 6);
}
