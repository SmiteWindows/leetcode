// https://leetcode.com/problems/integer-to-english-words/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn number_to_words(num: i32) -> String {
    fn words(
        num: usize,
        nineteen: &[&'static str],
        tens: &[&'static str],
        units: &[&'static str],
    ) -> Vec<&'static str> {
        if num < 20 {
            if num > 0 {
                vec![nineteen[num - 1]]
            } else {
                vec![]
            }
        } else if num < 100 {
            let d = num / 10;
            vec![
                if d > 1 { vec![tens[d - 2]] } else { vec![] },
                words(num % 10, nineteen, tens, units),
            ]
            .concat()
        } else if num < 1000 {
            vec![
                words(num / 100, nineteen, tens, units),
                vec![units[0]],
                words(num % 100, nineteen, tens, units),
            ]
            .concat()
        } else if num < 1_000_000 {
            vec![
                words(num / 1000, nineteen, tens, units),
                vec![units[1]],
                words(num % 1000, nineteen, tens, units),
            ]
            .concat()
        } else if num < 1_000_000_000 {
            vec![
                words(num / 1_000_000, nineteen, tens, units),
                vec![units[2]],
                words(num % 1_000_000, nineteen, tens, units),
            ]
            .concat()
        } else {
            vec![
                words(num / 1_000_000_000, nineteen, tens, units),
                vec![units[3]],
                words(num % 1_000_000_000, nineteen, tens, units),
            ]
            .concat()
        }
    }

    if num == 0 {
        return "Zero".to_string();
    }
    let nineteen: &'static str = "One Two Three Four Five Six Seven Eight Nine Ten Eleven Twelve Thirteen Fourteen Fifteen Sixteen Seventeen Eighteen Nineteen";
    let nineteen: Vec<&'static str> = nineteen.split_whitespace().collect();
    let tens: &'static str = "Twenty Thirty Forty Fifty Sixty Seventy Eighty Ninety";
    let tens: Vec<&'static str> = tens.split_whitespace().collect();
    let units: &'static str = "Hundred Thousand Million Billion";
    let units: Vec<&'static str> = units.split_whitespace().collect();
    words(num as usize, &nineteen, &tens, &units).join(" ")
}
// math string
#[test]
fn test1_273() {
    assert_eq!(
        number_to_words(123),
        String::from("One Hundred Twenty Three")
    );
    assert_eq!(
        number_to_words(12345),
        String::from("Twelve Thousand Three Hundred Forty Five")
    );
    assert_eq!(
        number_to_words(1234567),
        String::from("One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven")
    );
    assert_eq!(number_to_words(1234567891), String::from("One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"));
}
