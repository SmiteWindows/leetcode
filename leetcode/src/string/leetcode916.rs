// https://leetcode.com/problems/word-subsets/
// Runtime: 36 ms
// Memory Usage: 3.3 MB
pub fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
    let mut max_count = [0; 26];
    for s in b {
        let mut count = [0; 26];
        for b in s.bytes() {
            count[(b - b'a') as usize] += 1;
        }
        for i in 0..26 {
            max_count[i] = max_count[i].max(count[i]);
        }
    }
    let mut res = vec![];
    'a: for s in a {
        let mut count = [0; 26];
        for b in s.bytes() {
            count[(b - b'a') as usize] += 1;
        }
        for i in 0..26 {
            if count[i] < max_count[i] {
                continue 'a;
            };
        }
        res.push(s)
    }
    res
}
// string
#[test]
fn test1_916() {
    assert_eq!(
        word_subsets(
            vec![
                String::from("amazon"),
                String::from("apple"),
                String::from("facebook"),
                String::from("google"),
                String::from("leetcode")
            ],
            vec![String::from("e"), String::from("o")]
        ),
        vec![
            String::from("facebook"),
            String::from("google"),
            String::from("leetcode")
        ]
    );
    assert_eq!(
        word_subsets(
            vec![
                String::from("amazon"),
                String::from("apple"),
                String::from("facebook"),
                String::from("google"),
                String::from("leetcode")
            ],
            vec![String::from("l"), String::from("e")]
        ),
        vec![
            String::from("apple"),
            String::from("google"),
            String::from("leetcode")
        ]
    );
    assert_eq!(
        word_subsets(
            vec![
                String::from("amazon"),
                String::from("apple"),
                String::from("facebook"),
                String::from("google"),
                String::from("leetcode")
            ],
            vec![String::from("e"), String::from("oo")]
        ),
        vec![String::from("facebook"), String::from("google")]
    );
    assert_eq!(
        word_subsets(
            vec![
                String::from("amazon"),
                String::from("apple"),
                String::from("facebook"),
                String::from("google"),
                String::from("leetcode")
            ],
            vec![String::from("lo"), String::from("eo")]
        ),
        vec![String::from("google"), String::from("leetcode")]
    );
    assert_eq!(
        word_subsets(
            vec![
                String::from("amazon"),
                String::from("apple"),
                String::from("facebook"),
                String::from("google"),
                String::from("leetcode")
            ],
            vec![String::from("ec"), String::from("oc"), String::from("ceo")]
        ),
        vec![String::from("facebook"), String::from("leetcode")]
    );
}
