// https://leetcode-cn.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/
// Runtime: 12 ms
// Memory Usage: 2.4 MB
pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
    let n = n as usize;
    let mut dist = vec![vec![i32::MAX >> 2; n]; n];
    for (i, di) in dist.iter_mut().enumerate().take(n) {
        di[i] = 0;
    }
    for e in edges {
        let i = e[0] as usize;
        let j = e[1] as usize;
        let d = e[2];
        dist[i][j] = d;
        dist[j][i] = d;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }
    let mut min = (n, 0);
    for (i, di) in dist.iter().enumerate().take(n) {
        let count = di.iter().filter(|&&d| d <= distance_threshold).count() - 1;
        if count <= min.0 {
            min = (count, i);
        }
    }
    min.1 as i32
}
// graph
#[test]
fn test1_1334() {
    use leetcode_prelude::vec2;
    assert_eq!(
        find_the_city(4, vec2![[0, 1, 3], [1, 2, 1], [1, 3, 4], [2, 3, 1]], 4),
        3
    );
    assert_eq!(
        find_the_city(
            5,
            vec2![
                [0, 1, 2],
                [0, 4, 8],
                [1, 2, 3],
                [1, 4, 2],
                [2, 3, 1],
                [3, 4, 1]
            ],
            2
        ),
        0
    );
}
