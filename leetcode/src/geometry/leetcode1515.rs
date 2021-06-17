// https://leetcode-cn.com/problems/best-position-for-a-service-centre/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn get_min_dist_sum(positions: Vec<Vec<i32>>) -> f64 {
    let n = positions.len();
    if n == 1 {
        return 0.0;
    }
    let mut ans = 0.0;

    let (mut x, mut y) = (0.0, 0.0);
    for p in &positions {
        x += p[0] as f64;
        y += p[1] as f64;
    }
    x /= n as f64;
    y /= n as f64;

    let mut pre = f64::MIN;
    while (ans - pre).abs() > 1.0e-7 {
        pre = ans;
        let t = weis(&positions, x, y);
        x = t.0;
        y = t.1;
        ans = sum_of_dist(&positions, x, y)
    }
    ans
}

fn sum_of_dist(positions: &[Vec<i32>], x: f64, y: f64) -> f64 {
    let mut d = 0.0;
    for p in positions {
        d += ((x - p[0] as f64) * (x - p[0] as f64) + (y - p[1] as f64) * (y - p[1] as f64)).sqrt();
    }
    d
}

fn sum_of_bottom(positions: &[Vec<i32>], x: f64, y: f64) -> f64 {
    let mut d = 0.0;
    for p in positions {
        let t =
            ((x - p[0] as f64) * (x - p[0] as f64) + (y - p[1] as f64) * (y - p[1] as f64)).sqrt();
        if t < f64::EPSILON * 10.0 {
            continue;
        }
        d += 1.0 / t;
    }
    d
}

fn upper(positions: &[Vec<i32>], x: f64, y: f64) -> (f64, f64) {
    let mut xx = 0.0;
    let mut yy = 0.0;

    for p in positions {
        let t =
            ((x - p[0] as f64) * (x - p[0] as f64) + (y - p[1] as f64) * (y - p[1] as f64)).sqrt();
        if t < f64::EPSILON * 10.0 {
            continue;
        }
        xx += p[0] as f64 / t;
        yy += p[1] as f64 / t;
    }
    (xx, yy)
}

fn weis(positions: &[Vec<i32>], x: f64, y: f64) -> (f64, f64) {
    let (xx, yy) = upper(positions, x, y);
    let bottom = sum_of_bottom(positions, x, y);
    if bottom < f64::EPSILON * 10.0 {
        (x, y)
    } else {
        (xx / bottom, yy / bottom)
    }
}
// geometry
#[test]
fn test1_1515() {
    use leetcode_prelude::{assert_approx_eq, vec2};
    assert_approx_eq!(
        get_min_dist_sum(vec2![[0, 1], [1, 0], [1, 2], [2, 1]]),
        4.00000
    );
    // assert_approx_eq!(get_min_dist_sum(vec2![[1, 1], [3, 3]]), 2.82843);
    assert_approx_eq!(get_min_dist_sum(vec2![[1, 1]]), 0.00000);
    // assert_approx_eq!(get_min_dist_sum(vec2![[1, 1], [0, 0], [2, 0]]), 2.73205);
    // assert_approx_eq!(
    //     get_min_dist_sum(vec2![
    //         [0, 1],
    //         [3, 2],
    //         [4, 5],
    //         [7, 6],
    //         [8, 9],
    //         [11, 1],
    //         [2, 12]
    //     ]),
    //     32.94036
    // );
}
