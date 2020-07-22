// https://leetcode.com/problems/queens-that-can-attack-the-king/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashSet;
pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let cb = Chessboard::new(queens, king);
    for i in 0..8 {
        let mut step = 1;
        loop {
            let p = cb.attack(i, step);
            if cb.contains(&p) {
                if cb.queens.contains(&p) {
                    res.push(p);
                    break;
                } else {
                    step += 1;
                }
            } else {
                break;
            }
        }
    }
    res
}

struct Chessboard {
    directions: Vec<Vec<i32>>,
    queens: HashSet<Vec<i32>>,
    king: Vec<i32>,
}

impl Chessboard {
    fn new(queens_vec: Vec<Vec<i32>>, king: Vec<i32>) -> Self {
        let directions = vec![
            vec![1, 0],
            vec![-1, 0],
            vec![0, 1],
            vec![0, -1],
            vec![1, 1],
            vec![-1, 1],
            vec![1, -1],
            vec![-1, -1],
        ];
        let mut queens = HashSet::new();
        for queen in queens_vec {
            queens.insert(queen);
        }
        Self {
            king,
            queens,
            directions,
        }
    }

    fn contains(&self, point: &[i32]) -> bool {
        point[0] >= 0 && point[1] >= 0 && point[0] < 8 && point[1] < 8
    }

    fn attack(&self, i: usize, step: i32) -> Vec<i32> {
        let direction = &self.directions[i];
        let king = &self.king;
        vec![king[0] + direction[0] * step, king[1] + direction[1] * step]
    }
}
// array
#[test]
fn test1_1222() {
    let mut a = queens_attackthe_king(
        vec![
            vec![0, 1],
            vec![1, 0],
            vec![4, 0],
            vec![0, 4],
            vec![3, 3],
            vec![2, 4],
        ],
        vec![0, 0],
    );
    a.sort();
    assert_eq!(a, vec![vec![0, 1], vec![1, 0], vec![3, 3]]);
    let mut b = queens_attackthe_king(
        vec![
            vec![0, 0],
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![3, 5],
            vec![4, 4],
            vec![4, 5],
        ],
        vec![3, 3],
    );
    b.sort();
    assert_eq!(b, vec![vec![2, 2], vec![3, 4], vec![4, 4]]);
    let mut c = queens_attackthe_king(
        vec![
            vec![5, 6],
            vec![7, 7],
            vec![2, 1],
            vec![0, 7],
            vec![1, 6],
            vec![5, 1],
            vec![3, 7],
            vec![0, 3],
            vec![4, 0],
            vec![1, 2],
            vec![6, 3],
            vec![5, 0],
            vec![0, 4],
            vec![2, 2],
            vec![1, 1],
            vec![6, 4],
            vec![5, 4],
            vec![0, 0],
            vec![2, 6],
            vec![4, 5],
            vec![5, 2],
            vec![1, 4],
            vec![7, 5],
            vec![2, 3],
            vec![0, 5],
            vec![4, 2],
            vec![1, 0],
            vec![2, 7],
            vec![0, 1],
            vec![4, 6],
            vec![6, 1],
            vec![0, 6],
            vec![4, 3],
            vec![1, 7],
        ],
        vec![3, 4],
    );
    c.sort();
    assert_eq!(
        c,
        vec![
            vec![1, 4],
            vec![1, 6],
            vec![2, 3],
            vec![3, 7],
            vec![4, 3],
            vec![4, 5],
            vec![5, 4],
        ]
    );
}
