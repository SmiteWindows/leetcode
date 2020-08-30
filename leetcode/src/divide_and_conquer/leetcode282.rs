// https://leetcode-cn.com/problems/expression-add-operators/
// Runtime: 8 ms
// Memory Usage: 2.3 MB
pub fn add_operators(num: String, target: i32) -> Vec<String> {
    let target = target as i64;
    let num = num.as_bytes();
    let len = num.len();
    let mut path = Vec::new();
    let mut pathes = vec![];
    if len > 0 {
        let mut v = num[0] as i64 - 48;
        path.push(num[0]);
        dfs(&num[1..], target, 0, v, &mut path, &mut pathes);
        if v != 0 {
            for i in 1..len {
                path.push(num[i]);
                v = v * 10 + num[i] as i64 - 48;
                dfs(&num[i + 1..], target, 0, v, &mut path, &mut pathes);
            }
        }
    }
    pathes
}

fn dfs(num: &[u8], target: i64, v1: i64, v2: i64, path: &mut Vec<u8>, pathes: &mut Vec<String>) {
    let len = num.len();
    if len == 0 {
        if v1 + v2 == target {
            pathes.push(unsafe { String::from_utf8_unchecked(path.clone()) });
        }
        return;
    }
    let sign_idx = path.len();
    let mut v = num[0] as i64 - 48;
    path.push(b'+');
    path.push(num[0]);

    let mut try_ops = |path: &mut Vec<u8>, v, i| {
        path[sign_idx] = b'+';
        dfs(&num[i..], target, v1 + v2, v, path, pathes);
        path[sign_idx] = b'-';
        dfs(&num[i..], target, v1 + v2, -v, path, pathes);
        path[sign_idx] = b'*';
        dfs(&num[i..], target, v1, v2 * v, path, pathes);
    };

    try_ops(path, v, 1);
    if v != 0 {
        for (i, &ni) in num.iter().enumerate().take(len).skip(1) {
            path.push(ni);
            v = v * 10 + ni as i64 - 48;
            try_ops(path, v, i + 1);
        }
    }
    path.resize(sign_idx, 0);
}
// divide_and_conquer
#[test]
fn test1_282() {
    use leetcode_prelude::{assert_eq_sorted, vec_string};
    assert_eq_sorted!(
        add_operators(String::from("123"), 6),
        vec_string!["1+2+3", "1*2*3"]
    );
    assert_eq_sorted!(
        add_operators(String::from("232"), 8),
        vec_string!["2*3+2", "2+3*2"]
    );
    assert_eq_sorted!(
        add_operators(String::from("105"), 5),
        vec_string!["1*0+5", "10-5"]
    );
    assert_eq_sorted!(
        add_operators(String::from("00"), 0),
        vec_string!["0+0", "0-0", "0*0"]
    );
    assert_eq_sorted!(
        add_operators(String::from("3456237490"), 9191),
        vec![] as Vec<String>
    );
}
