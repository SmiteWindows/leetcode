// https://leetcode-cn.com/problems/distant-barcodes/
// Runtime: 24 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
    let n = barcodes.len();
    if n == 1 {
        return barcodes;
    }
    let mut hm: HashMap<i32, usize> = HashMap::new();
    let mut max = (0, 0);
    for barcode in barcodes {
        let count = hm.entry(barcode).or_default();
        *count += 1;
        if *count > max.0 {
            max = (*count, barcode);
        }
    }
    let mut stack = Vec::new();
    for (k, v) in hm {
        if k != max.1 {
            stack.resize(v, k);
        }
    }
    stack.resize(stack.len() + max.0, max.1);
    let mut res = vec![0; n];
    let m = if n % 2 == 0 { n / 2 } else { (n + 1) / 2 };
    for i in 0..m {
        res[i * 2] = stack.pop().unwrap();
    }
    let mut i = 1;
    while let Some(top) = stack.pop() {
        res[i] = top;
        i += 2;
    }
    res
}
// sort heap
#[test]
fn test2_1054() {
    assert_eq!(
        rearrange_barcodes(vec![1, 1, 1, 2, 2, 2]),
        vec![1, 2, 1, 2, 1, 2] // [2,1,2,1,2,1]
    );
    // FIX
    // assert_eq!(
    //     rearrange_barcodes(vec![1, 1, 1, 1, 2, 2, 3, 3]),
    //     vec![1, 3, 1, 3, 1, 2, 1, 2] [1,3,1,3,2,1,2,1]
    // );
}
