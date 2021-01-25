// https://leetcode-cn.com/problems/grumpy-bookstore-owner/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
    let mut sum = 0;
    let mut max = 0;
    let n = customers.len();
    for i in 0..n {
        if grumpy[i] == 0 {
            sum += customers[i];
        }
    }
    let x = x as usize;
    for i in 0..x {
        if grumpy[i] == 1 {
            sum += customers[i];
        }
    }
    max = max.max(sum);
    for i in x..n {
        if grumpy[i] == 1 {
            sum += customers[i];
        }
        if grumpy[i - x] == 1 {
            sum -= customers[i - x];
        }
        max = max.max(sum);
    }
    max
}
// sliding_window array
#[test]
fn test1_1052() {
    assert_eq!(
        max_satisfied(
            vec![1, 0, 1, 2, 1, 1, 7, 5],
            vec![0, 1, 0, 1, 0, 1, 0, 1],
            3
        ),
        16
    );
}
