// https://leetcode-cn.com/problems/range-sum-query-mutable/
// Runtime: 0 ms
// Memory Usage: 9.5 MB
use std::iter::successors;

struct BitTree {
    tree: Vec<i32>,
    data: Vec<i32>,
    n: usize,
}

impl BitTree {
    fn new(n: usize) -> Self {
        let tree = vec![0; n];
        let data = vec![0; n];
        Self { tree, data, n }
    }

    fn get(&self, i: usize) -> i32 {
        self.data[i]
    }

    fn sum(&self, i: usize) -> i32 {
        let mut res = 0;
        let down_iter = successors(Some(i), |&i| {
            let j = i & (i + 1);
            if j > 0 {
                Some(j - 1)
            } else {
                None
            }
        });
        for j in down_iter {
            res += self.tree[j];
        }
        res
    }

    fn add(&mut self, i: usize, v: i32) {
        self.data[i] += v;
        let n = self.n;
        let up_iter = successors(Some(i), |&i| {
            let j = i | (i + 1);
            if j < n {
                Some(j)
            } else {
                None
            }
        });
        for j in up_iter {
            self.tree[j] += v;
        }
    }
}

struct NumArray {
    bit_tree: BitTree,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut bit_tree = BitTree::new(n);
        for (i, &num) in nums.iter().enumerate().take(n) {
            bit_tree.add(i, num);
        }
        Self { bit_tree }
    }

    fn update(&mut self, i: i32, val: i32) {
        let i = i as usize;
        self.bit_tree.add(i as usize, val - self.bit_tree.get(i))
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let i = i as usize;
        let j = j as usize;
        if i > 0 {
            self.bit_tree.sum(j) - self.bit_tree.sum(i - 1)
        } else {
            self.bit_tree.sum(j)
        }
    }
}
/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(i, val);
 * let ret_2: i32 = obj.sum_range(i, j);
 */
// binary_indexed_tree segment_tree
#[test]
fn test2_307() {
    let mut obj = NumArray::new(vec![1, 3, 5]);
    assert_eq!(obj.sum_range(0, 2), 9);
    obj.update(1, 2);
    assert_eq!(obj.sum_range(0, 2), 8);
}
