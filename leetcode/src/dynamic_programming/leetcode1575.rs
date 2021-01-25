// https://leetcode-cn.com/problems/count-all-possible-routes/
// Runtime: 300 ms
// Memory Usage: 3.5 MB
use std::collections::HashMap;
const MOD: i64 = 1_000_000_007;
pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
    let n = locations.len();
    let mut memo: HashMap<(usize, i32), i64> = HashMap::new();
    let start = start as usize;
    let finish = finish as usize;
    memo.insert((start, fuel), 1);
    ((0..=fuel)
        .map(|f| dp(finish, f, &mut memo, &locations, fuel, n))
        .sum::<i64>()
        % MOD) as i32
}

fn dp(
    i: usize,
    fuel: i32,
    memo: &mut HashMap<(usize, i32), i64>,
    locations: &[i32],
    max_fuel: i32,
    n: usize,
) -> i64 {
    if fuel > max_fuel {
        return 0;
    }
    if let Some(&res) = memo.get(&(i, fuel)) {
        return res;
    }
    let mut res = 0;
    for j in 0..n {
        if i != j {
            let cost = (locations[i] - locations[j]).abs();
            res += dp(j, fuel + cost, memo, locations, max_fuel, n);
            res %= MOD;
        }
    }
    memo.insert((i, fuel), res);
    res
}
// dynamic_programming
#[test]
fn test1_1575() {
    assert_eq!(count_routes(vec![2, 3, 6, 8, 4], 1, 3, 5), 4);
    assert_eq!(count_routes(vec![4, 3, 1], 1, 0, 6), 5);
    assert_eq!(count_routes(vec![5, 2, 1], 0, 2, 3), 0);
    assert_eq!(count_routes(vec![2, 1, 5], 0, 0, 3), 2);
    assert_eq!(count_routes(vec![1, 2, 3], 0, 2, 40), 615088286);
}
