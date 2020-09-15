// https://leetcode-cn.com/problems/24-game/
// TODO
pub fn judge_point24(nums: Vec<i32>) -> bool {
    let nums: Vec<f64> = nums.into_iter().map(|x| x as f64).collect();
    let a = nums[0];
    let b = nums[1];
    let c = nums[2];
    let d = nums[3];
    let ab_cd = values(&values(&[a], &[b]), &values(&[c], &[d]));
    let ac_bd = values(&values(&[a], &[c]), &values(&[b], &[d]));
    let ad_bc = values(&values(&[b], &[d]), &values(&[b], &[c]));
    let a_bcd = values(&[a], &rotate(&[b], &[c], &[d]));
    let b_acd = values(&[b], &rotate(&[a], &[c], &[d]));
    let c_abd = values(&[c], &rotate(&[a], &[b], &[d]));
    let d_abc = values(&[d], &rotate(&[a], &[b], &[c]));

    vec![ab_cd, ac_bd, ad_bc, a_bcd, b_acd, c_abd, d_abc]
        .into_iter()
        .flatten()
        .any(|x| (x - 24.0).abs() < 0.000001)
}

fn rotate(a: &[f64], b: &[f64], c: &[f64]) -> Vec<f64> {
    let ab_c = values(&values(a, b), c);
    let ac_b = values(&values(a, c), b);
    let bc_a = values(&values(b, c), a);
    vec![ab_c, ac_b, bc_a].into_iter().flatten().collect()
}

fn values(a: &[f64], b: &[f64]) -> Vec<f64> {
    let mut res = vec![];
    for i in 0..a.len() {
        for j in 0..b.len() {
            let x = a[i];
            let y = b[j];
            res.push(x + y);
            res.push(x - y);
            res.push(x * y);
            res.push(x / y);
            res.push(y - x);
            res.push(y / x);
        }
    }
    res
}
// depth_first_search
#[test]
fn test1_679() {
    assert_eq!(judge_point24(vec![4, 1, 8, 7]), true);
    assert_eq!(judge_point24(vec![1, 2, 1, 2]), false);
}
