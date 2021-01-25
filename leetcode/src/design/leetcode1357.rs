// https://leetcode-cn.com/problems/apply-discount-every-n-orders/
// Runtime: 84 ms
// Memory Usage: 13.4 MB
use std::collections::HashMap;
struct Cashier {
    n: usize,
    index: usize,
    discount: f64,
    inventory: HashMap<i32, f64>,
}

impl Cashier {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let n = n as usize;
        let discount = (100 - discount) as f64 / 100.0;
        let mut inventory = HashMap::new();
        for (id, price) in products.into_iter().zip(prices.into_iter()) {
            inventory.insert(id, price as f64);
        }
        Self {
            n,
            index: 0,
            inventory,
            discount,
        }
    }

    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        let mut res = 0.0;
        for (id, amount) in product.into_iter().zip(amount.into_iter()) {
            res += self.inventory[&id] * amount as f64;
        }
        self.index += 1;
        if self.index == self.n {
            self.index = 0;
            res * self.discount
        } else {
            res
        }
    }
}
/**
 * Your Cashier object will be instantiated and called as such:
 * let obj = Cashier::new(n, discount, products, prices);
 * let ret_1: f64 = obj.get_bill(product, amount);
 */
// design
#[test]
fn test() {
    use leetcode_prelude::assert_approx_eq;
    let mut cashier = Cashier::new(
        3,
        50,
        vec![1, 2, 3, 4, 5, 6, 7],
        vec![100, 200, 300, 400, 300, 200, 100],
    );
    // assert!((cashier.get_bill(vec![1, 2], vec![1, 2]) - 500.0).abs() <= f64::EPSILON);
    // assert!((cashier.get_bill(vec![3, 7], vec![10, 10]) - 4000.0).abs() <= f64::EPSILON);
    // assert!(
    //     (cashier.get_bill(vec![1, 2, 3, 4, 5, 6, 7], vec![1, 1, 1, 1, 1, 1, 1]) - 800.0).abs()
    //         <= f64::EPSILON
    // );
    // assert!((cashier.get_bill(vec![4], vec![10]) - 4000.0).abs() <= f64::EPSILON);
    // assert!((cashier.get_bill(vec![7, 3], vec![10, 10]) - 4000.0).abs() <= f64::EPSILON);
    // assert!(
    //     (cashier.get_bill(vec![7, 5, 3, 1, 6, 4, 2], vec![10, 10, 10, 9, 9, 9, 7]) - 7350.0).abs()
    //         <= f64::EPSILON
    // );
    // assert!((cashier.get_bill(vec![2, 3, 5], vec![5, 3, 2]) - 2500.0).abs() <= f64::EPSILON);
    assert_approx_eq!(cashier.get_bill(vec![1, 2], vec![1, 2]), 500.0);
    assert_approx_eq!(cashier.get_bill(vec![3, 7], vec![10, 10]), 4000.0);
    assert_approx_eq!(
        cashier.get_bill(vec![1, 2, 3, 4, 5, 6, 7], vec![1, 1, 1, 1, 1, 1, 1]),
        800.0
    );
    assert_approx_eq!(cashier.get_bill(vec![4], vec![10]), 4000.0);
    assert_approx_eq!(cashier.get_bill(vec![7, 3], vec![10, 10]), 4000.0);
    assert_approx_eq!(
        cashier.get_bill(vec![7, 5, 3, 1, 6, 4, 2], vec![10, 10, 10, 9, 9, 9, 7]),
        7350.0
    );
    assert_approx_eq!(cashier.get_bill(vec![2, 3, 5], vec![5, 3, 2]), 2500.0);
}
