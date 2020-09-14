// https://leetcode-cn.com/problems/minimize-malware-spread/
pub fn min_malware_spread(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
    todo!()
}
// union_find depth_first_search
#[test]
#[ignore]
fn test1_924() {
    use leetcode_prelude::vec2;
    assert_eq!(
        min_malware_spread(vec2![[1, 1, 0], [1, 1, 0], [0, 0, 1]], vec![0, 1]),
        0
    );
    assert_eq!(
        min_malware_spread(vec2![[1, 0, 0], [0, 1, 0], [0, 0, 1]], vec![0, 2]),
        0
    );
    assert_eq!(
        min_malware_spread(vec2![[1, 1, 1], [1, 1, 1], [1, 1, 1]], vec![1, 2]),
        1
    );
}
