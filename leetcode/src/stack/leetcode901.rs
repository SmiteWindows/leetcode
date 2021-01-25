// https://leetcode-cn.com/problems/online-stock-span/
// Runtime: 32 ms
// Memory Usage: 5.4 MB
struct StockSpanner {
    stack: Vec<(i32, usize)>,
}

impl StockSpanner {
    fn new() -> Self {
        Self { stack: vec![] }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut res = 1;
        while let Some(top) = self.stack.pop() {
            if top.0 > price {
                self.stack.push(top);
                break;
            } else {
                res += top.1;
            }
        }
        self.stack.push((price, res));
        res as i32
    }
}
/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */
// stack
#[test]
fn test1_901() {
    let mut obj = StockSpanner::new();
    assert_eq!(obj.next(100), 1);
    assert_eq!(obj.next(80), 1);
    assert_eq!(obj.next(60), 1);
    assert_eq!(obj.next(70), 2);
    assert_eq!(obj.next(60), 1);
    assert_eq!(obj.next(75), 4);
    assert_eq!(obj.next(85), 6);
}
