// https://leetcode-cn.com/problems/xor-queries-of-a-subarray/
// Runtime: 24 ms
// Memory Usage: 3.8 MB
pub fn xor_queries(mut arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = arr.len();
    for i in 1..n {
        arr[i] ^= arr[i - 1];
    }
    let mut res = vec![];
    for query in queries {
        let l = query[0] as usize;
        let r = query[1] as usize;
        let x = if l > 0 { arr[r] ^ arr[l - 1] } else { arr[r] };
        res.push(x);
    }
    res
}
// bit_manipulation
#[test]
fn test1_1310() {
    use leetcode_prelude::vec2;
    assert_eq!(
        xor_queries(vec![1, 3, 4, 8], vec2![[0, 1], [1, 2], [0, 3], [3, 3]]),
        vec![2, 7, 14, 8]
    );
    assert_eq!(
        xor_queries(vec![4, 8, 2, 10], vec2![[2, 3], [1, 3], [0, 0], [0, 3]]),
        vec![8, 0, 4, 4]
    );
}
