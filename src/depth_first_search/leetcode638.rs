// https://leetcode.com/problems/shopping-offers/
pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
    todo!()
}
// dynamic_programming depth_first_search
#[test]
#[ignore]
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
