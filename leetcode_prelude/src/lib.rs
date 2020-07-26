#![warn(rust_2018_idioms)]
pub use list::ListNode;
pub use tree::TreeNode;

mod list;
mod tree;

/// Create a Vec<String>
#[macro_export]
macro_rules! vec_string {
    ($($e:expr), *) => {vec![$($e.to_string()), *]};
}

#[macro_export]
macro_rules! assert_eq_sorted {
    ($left:expr, $right:expr) => ({
        let (mut v1, mut v2) = ($left, $right);
        v1.sort();
        v2.sort();
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
        assert_eq!(vec!["a".to_owned(), "b".to_owned()], vec_string!["a", "b"]);
    }

    #[test]
    fn assert_eq_sorted() {
        assert_eq_sorted!(vec![1, 2], vec![2, 1]);
    }
}
