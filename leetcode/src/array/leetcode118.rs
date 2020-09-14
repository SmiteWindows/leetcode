// https://leetcode-cn.com/problems/pascals-triangle/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    for i in 0..num_rows as usize {
        res.push(vec![]);
        for j in 0..=i {
            if j == 0 || j == i {
                res[i].push(1);
            } else {
                let prev = &res[i - 1];
                let sum = prev[j - 1] + prev[j];
                res[i].push(sum);
            }
        }
    }
    res
}
// array
#[test]
fn test1_118() {
    use leetcode_prelude::vec2;
    assert_eq!(
        generate(5),
        vec2![[1], [1, 1], [1, 2, 1], [1, 3, 3, 1], [1, 4, 6, 4, 1]]
    );
}
