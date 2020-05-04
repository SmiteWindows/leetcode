// https://leetcode.com/problems/distant-barcodes/
pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
    todo!()
}
// sort heap
#[test]
#[ignore]
fn test1_1054() {
    assert_eq!(
        rearrange_barcodes(vec![1, 1, 1, 2, 2, 2]),
        vec![2, 1, 2, 1, 2, 1]
    );
    assert_eq!(
        rearrange_barcodes(vec![1, 1, 1, 1, 2, 2, 3, 3]),
        vec![1, 3, 1, 3, 2, 1, 2, 1]
    );
}
