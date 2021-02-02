// https://leetcode-cn.com/problems/majority-element-ii/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut pairs: Vec<(i32, usize)> = Vec::new();
    for &x in &nums {
        if pairs.iter().any(|p| p.0 == x) {
            pairs = pairs
                .into_iter()
                .map(|p| (p.0, if p.0 == x { p.1 + 1 } else { p.1 }))
                .collect();
        } else if pairs.len() < 2 {
            pairs.push((x, 1));
        } else {
            pairs = pairs.into_iter().map(|p| (p.0, p.1 - 1)).collect();
            pairs = pairs.into_iter().filter(|p| p.1 > 0).collect();
        }
    }
    let mut res = Vec::new();
    for pair in pairs {
        let sum = nums
            .iter()
            .fold(0, |acc, &x| if x == pair.0 { acc + 1 } else { acc });
        if sum > n / 3 {
            res.push(pair.0);
        }
    }
    res
}
// array
#[test]
fn test1_229() {
    assert_eq!(majority_element(vec![3, 2, 3]), vec![3]);
    assert_eq!(majority_element(vec![1, 1, 1, 3, 3, 2, 2, 2]), vec![1, 2]);
}
