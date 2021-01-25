#![warn(rust_2018_idioms)]
mod list;
mod tree;
pub use crate::{list::ListNode, tree::TreeNode};

/// Create a Vec<String>
#[macro_export]
macro_rules! vec_string {
    ($($e:tt), *) => {vec![$($e.to_string()), *] as Vec<String>};
}

/// Create a Vec<Vec<String>>
#[macro_export]
macro_rules! vec2_string {
    ($($e:tt), *) => {vec![$($crate::vec_string!$e), *] as Vec<Vec<String>>};
}
/// Create a Vec<Option<Rc<RefCell<TreeNode>>>>
#[macro_export]
macro_rules! vec_btree {
    ($($e:tt), *) => {vec![$($crate::btree!$e), *]};
}
/// Create a Vec<Option<Box<ListNode>>
#[macro_export]
macro_rules! vec_list {
    ($($e:tt), *) => {vec![$($crate::list!$e), *]};
}

/// Create a Vec<Vec<char>>
#[macro_export]
macro_rules! vec2_char {
    ($($e:tt), *) => {vec![$(vec!$e), *]};
}

/// Create a Vec<Vec<i32>>
#[macro_export]
macro_rules! vec2 {
    ($($e:tt), *) => {vec![$(vec!$e), *] as Vec<Vec<i32>>};
}

#[macro_export]
macro_rules! assert_eq_sorted {
    ($left:expr, $right:expr) => ({
        let (mut v1, mut v2) = ($left, $right);
        v1.sort_unstable();
        v2.sort_unstable();
        assert_eq!(v1, v2)
    });
    ($left:expr, $right:expr,) => ({
        assert_eq_sorted!($left, $right)
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        let (mut v1, mut v2) = ($left, $right);
        v1.sort();
        v2.sort();
        assert_eq!(v1, v2, $($arg)+)
    });
}

#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr,$b:expr) => {
        let eps = f64::EPSILON;
        let (a, b) = ($a as f64, $b as f64);
        assert!(
            (a - b).abs() <= eps,
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            a,
            b,
            eps,
            (a - b).abs()
        );
    };
    ($a:expr,$b:expr,$eps:expr) => {
        let (a, b, expr) = ($a as f64, $b as f64, $expr as f64);
        assert!(
            (a - b).abs() <= eps,
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            a,
            b,
            eps,
            (a - b).abs()
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_string() {
        assert_eq!(
            vec!["a".to_string(), "b".to_string()],
            vec_string!["a", "b"]
        );
    }

    #[test]
    fn test_vec2_char() {
        assert_eq!(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            vec2_char![
                ['A', 'B', 'C', 'E'],
                ['S', 'F', 'C', 'S'],
                ['A', 'D', 'E', 'E']
            ]
        );
    }

    #[test]
    fn test_vec2_string() {
        assert_eq!(
            vec![
                vec![
                    ("John").to_string(),
                    ("johnsmith@mail.com").to_string(),
                    ("john00@mail.com").to_string(),
                ],
                vec![("John").to_string(), ("johnnybravo@mail.com").to_string()],
                vec![
                    ("John").to_string(),
                    ("johnsmith@mail.com").to_string(),
                    ("john_newyork@mail.com").to_string(),
                ],
                vec![("Mary").to_string(), ("mary@mail.com").to_string()],
            ],
            vec2_string![
                ["John", "johnsmith@mail.com", "john00@mail.com"],
                ["John", "johnnybravo@mail.com"],
                ["John", "johnsmith@mail.com", "john_newyork@mail.com"],
                ["Mary", "mary@mail.com"]
            ]
        );
    }

    #[test]
    fn test_vec2() {
        assert_eq!(
            vec![
                vec![0, 1],
                vec![1, 0],
                vec![4, 0],
                vec![0, 4],
                vec![3, 3],
                vec![2, 4],
            ],
            vec2![[0, 1], [1, 0], [4, 0], [0, 4], [3, 3], [2, 4]]
        );
    }

    #[test]
    fn assert_eq_sorted() {
        assert_eq_sorted!(vec![1, 2], vec![2, 1]);
    }
}
