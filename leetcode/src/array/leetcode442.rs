// https://leetcode-cn.com/problems/find-all-duplicates-in-an-array/
// Runtime: 28 ms
// Memory Usage: 2.5 MB
pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    let n = nums.len();
    for i in 0..n {
        let x = nums[i];
        let index = (x.abs() - 1) as usize;
        if nums[index] < 0 {
            res.push(x.abs());
        } else {
            nums[index] *= -1;
        }
    }
    res
}
// array
#[test]
fn test1_442() {
    assert_eq!(find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]), vec![2, 3]);
}
