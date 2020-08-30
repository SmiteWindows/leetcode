// https://leetcode-cn.com/problems/create-target-array-in-the-given-order/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    let n = nums.len();
    for i in 0..n {
        let e = nums[i];
        let j = index[i] as usize;
        res.insert(j, e);
    }
    res
}
// array
#[test]
fn test1_1389() {
    assert_eq!(
        create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
        vec![0, 4, 1, 3, 2]
    );
    assert_eq!(
        create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]),
        vec![0, 1, 2, 3, 4]
    );
    assert_eq!(create_target_array(vec![1], vec![0]), vec![1]);
}
