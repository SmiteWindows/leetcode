// https://leetcode-cn.com/problems/rle-iterator/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
#[allow(clippy::upper_case_acronyms)]
struct RLEIterator {
    prefix: Vec<usize>,
    values: Vec<i32>,
    index: usize,
    size: usize,
}

impl RLEIterator {
    fn new(a: Vec<i32>) -> Self {
        let mut prefix = Vec::new();
        let mut values = Vec::new();
        let n = a.len();
        let index = 0;
        let mut prev = 0;
        for i in 0..n / 2 {
            let time = a[i * 2] as usize;
            if time != 0 {
                prev += time;
                prefix.push(prev);
                values.push(a[i * 2 + 1]);
            }
        }
        let size = values.len();
        Self {
            prefix,
            values,
            index,
            size,
        }
    }

    fn next(&mut self, n: i32) -> i32 {
        self.index += n as usize;
        match self.prefix.binary_search(&self.index) {
            Ok(i) => self.values[i],
            Err(i) => {
                if i < self.size {
                    self.values[i]
                } else {
                    -1
                }
            }
        }
    }
}
/**
 * Your RLEIterator object will be instantiated and called as such:
 * let obj = RLEIterator::new(A);
 * let ret_1: i32 = obj.next(n);
 */
// array
#[test]
fn test() {
    let mut obj = RLEIterator::new(vec![3, 8, 0, 9, 2, 5]);
    assert_eq!(obj.next(2), 8);
    assert_eq!(obj.next(1), 8);
    assert_eq!(obj.next(1), 5);
    assert_eq!(obj.next(2), -1);
}
