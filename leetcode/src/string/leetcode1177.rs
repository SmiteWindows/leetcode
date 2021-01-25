// https://leetcode-cn.com/problems/can-make-palindrome-from-substring/
// Runtime: 96 ms
// Memory Usage: 9.8 MB
pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let n = s.len();
    let mut prefix: Vec<u32> = vec![0; n + 1];
    for (i, c) in s.char_indices() {
        prefix[i + 1] = prefix[i] ^ (1 << (c as u32 - 'a' as u32));
    }
    let mut res = vec![];
    for q in queries {
        let left = q[0] as usize;
        let right = q[1] as usize + 1;
        let k = q[2] as u32;
        res.push(k * 2 >= (prefix[right] ^ prefix[left]).count_ones() - 1);
    }
    res
}
// array string
#[test]
fn test1_1177() {
    use leetcode_prelude::vec2;
    assert_eq!(
        can_make_pali_queries(
            "abcda".to_string(),
            vec2![[3, 3, 0], [1, 2, 0], [0, 3, 1], [0, 3, 2], [0, 4, 1]]
        ),
        vec![true, false, false, true, true]
    );
}
