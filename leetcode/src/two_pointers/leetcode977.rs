// https://leetcode-cn.com/problems/squares-of-a-sorted-array/
pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
    todo!()
}
// two_pointers array
#[test]
#[ignore]
fn test1_977() {
    assert_eq!(
        sorted_squares(vec![-4, -1, 0, 3, 10]),
        vec![0, 1, 9, 16, 100]
    );
    assert_eq!(
        sorted_squares(vec![-7, -3, 2, 3, 11]),
        vec![4, 9, 9, 49, 121]
    );
}
// 我们的策略就是从前向后遍历数组中的非负数部分，并且反向遍历数组中的负数部分。

// 算法

// 我们可以使用两个指针分别读取数组的非负部分与负数部分 —— 指针 i 反向读取负数部分，指针 j 正向读取非负数部分。

// 那么，现在我们就在使用两个指针分别读取两个递增的数组了（按元素的平方排序）。接下来，我们可以使用双指针的技巧合并这两个数组。
