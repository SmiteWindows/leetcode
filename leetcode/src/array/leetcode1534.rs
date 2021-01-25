// https://leetcode-cn.com/problems/count-good-triplets/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let n = arr.len();
    let mut res = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if (arr[i] - arr[j]).abs() <= a
                    && (arr[j] - arr[k]).abs() <= b
                    && (arr[i] - arr[k]).abs() <= c
                {
                    res += 1;
                }
            }
        }
    }
    res
}
// array
#[test]
fn test1_1534() {
    assert_eq!(count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3), 4);
    assert_eq!(count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1), 0);
}
