// https://leetcode.com/problems/iterator-for-combination/
// Runtime: 4 ms
// Memory Usage: 4 MB
struct CombinationIterator {
    combinations: Vec<String>,
    index: usize,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {
    fn new(characters: String, combination_length: i32) -> Self {
        let combination_length = combination_length as usize;
        let n = characters.len();
        let mut indexes = vec![];
        let mut combinations = vec![];
        let s = characters.chars().collect::<Vec<_>>();
        Self::dfs(
            0,
            &mut indexes,
            &mut combinations,
            &s,
            combination_length,
            n,
        );
        Self {
            combinations,
            index: 0,
        }
    }

    fn dfs(
        start: usize,
        indexes: &mut Vec<usize>,
        combinations: &mut Vec<String>,
        s: &[char],
        m: usize,
        n: usize,
    ) {
        if indexes.len() == m {
            let t = indexes.iter().map(|&i| s[i]).collect();
            combinations.push(t);
        } else {
            for i in start..n {
                indexes.push(i);
                Self::dfs(i + 1, indexes, combinations, s, m, n);
                indexes.pop();
            }
        }
    }

    fn next(&mut self) -> String {
        let res = self.combinations[self.index].to_string();
        self.index += 1;
        res
    }

    fn has_next(&self) -> bool {
        self.index < self.combinations.len()
    }
}
/**
 * Your CombinationIterator object will be instantiated and called as such:
 * let obj = CombinationIterator::new(characters, combinationLength);
 * let ret_1: String = obj.next();
 * let ret_2: bool = obj.has_next();
 */
// design backtracking
#[test]
fn test2_1286() {
    let mut iter = CombinationIterator::new("abc".to_string(), 2);
    assert_eq!(iter.has_next(), true);
    assert_eq!(iter.next(), "ab".to_string());
    assert_eq!(iter.has_next(), true);
    assert_eq!(iter.next(), "ac".to_string());
    assert_eq!(iter.has_next(), true);
    assert_eq!(iter.next(), "bc".to_string());
    assert_eq!(iter.has_next(), false);
}
