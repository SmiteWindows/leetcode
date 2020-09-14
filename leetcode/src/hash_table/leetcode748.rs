// https://leetcode-cn.com/problems/shortest-completing-word/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
    let mut min: Option<String> = None;
    let lowercase = license_plate
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>()
        .to_lowercase();
    let count_of_lowercase = Count::new(&lowercase);
    for word in words {
        if Count::new(&word).completes(&count_of_lowercase) {
            if let Some(ref s) = min {
                if word.len() < s.len() {
                    min = Some(word);
                }
            } else {
                min = Some(word);
            }
        }
    }
    min.unwrap()
}

struct Count {
    v: Vec<i32>,
}

impl Count {
    fn new(s: &str) -> Self {
        let mut v = vec![0; 256];
        for c in s.chars() {
            v[c as usize] += 1;
        }
        Self { v }
    }

    fn completes(&self, other: &Self) -> bool {
        for i in 0..26 {
            let c = (b'a' + i) as usize;
            if self.v[c] < other.v[c] {
                return false;
            }
        }
        true
    }
}
// hash_table
#[test]
fn test1_748() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        shortest_completing_word(
            "1s3 PSt".to_string(),
            vec_string!["step", "steps", "stripe", "stepple"]
        ),
        "steps".to_string()
    );
    assert_eq!(
        shortest_completing_word(
            "1s3 456".to_string(),
            vec_string!["looks", "pest", "stew", "show"]
        ),
        "pest".to_string()
    );
}
