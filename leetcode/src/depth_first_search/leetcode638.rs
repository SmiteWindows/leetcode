// https://leetcode.com/problems/shopping-offers/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
    let n = price.len();
    let m = special.len();
    let mut memo = HashMap::new();
    memo.insert(vec![0; n], 0);
    dfs(needs, &mut memo, &price, &special, n, m)
}

fn dfs(
    needs: Vec<i32>,
    memo: &mut HashMap<Vec<i32>, i32>,
    price: &[i32],
    special: &[Vec<i32>],
    n: usize,
    m: usize,
) -> i32 {
    if let Some(&res) = memo.get(&needs) {
        return res;
    }
    let mut res = needs
        .iter()
        .zip(price.iter())
        .map(|(a, b)| a * b)
        .sum::<i32>();
    'special: for s in special.iter() {
        for i in 0..n {
            if s[i] > needs[i] {
                continue 'special;
            }
        }
        let mut needs = needs.to_vec();
        for i in 0..n {
            needs[i] -= s[i];
        }
        res = res.min(dfs(needs, memo, price, special, n, m) + s[n]);
    }
    memo.insert(needs, res);
    res
}
// dynamic_programming depth_first_search
#[test]
fn test2_638() {
    assert_eq!(
        shopping_offers(vec![2, 5], vec![vec![3, 0, 5], vec![1, 2, 10]], vec![3, 2]),
        14
    );
    assert_eq!(
        shopping_offers(
            vec![2, 3, 4],
            vec![vec![1, 1, 0, 4], vec![2, 2, 1, 9]],
            vec![1, 2, 1]
        ),
        11
    );
}
