// https://leetcode.com/problems/number-of-ways-to-reorder-array-to-get-same-bst/
// Runtime: 60 ms
// Memory Usage: 8.7 MB
pub fn num_of_ways(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut table = vec![];
    for i in 0..=n {
        table.push(vec![1; i + 1]);
        for j in 1..i {
            table[i][j] = (table[i - 1][j - 1] + table[i - 1][j]) % MOD;
        }
    }
    ways(nums, &table) as i32 - 1
}

fn ways(nums: Vec<i32>, table: &[Vec<i64>]) -> i64 {
    if nums.is_empty() {
        return 1;
    }
    let root = nums[0];
    let left: Vec<i32> = nums.iter().filter(|&&x| x < root).copied().collect();
    let right: Vec<i32> = nums.iter().filter(|&&x| x > root).copied().collect();
    let l = left.len();
    let r = right.len();
    let mut res = table[l + r][l];
    res %= MOD;
    res *= ways(right, table);
    res %= MOD;
    res *= ways(left, table);
    res %= MOD;
    res
}
// dynamic_programming
#[test]
fn test1_1569() {
    assert_eq!(num_of_ways(vec![2, 1, 3]), 1);
    assert_eq!(num_of_ways(vec![3, 4, 5, 1, 2]), 5);
    assert_eq!(num_of_ways(vec![1, 2, 3]), 0);
    assert_eq!(num_of_ways(vec![3, 1, 2, 5, 4, 6]), 19);
    assert_eq!(
        num_of_ways(vec![
            9, 4, 2, 1, 3, 6, 5, 7, 8, 14, 11, 10, 12, 13, 16, 15, 17, 18
        ]),
        216212978
    );
}
