use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn is_vowel(c: char) -> bool {
    let vowels = "aeiou";
    vowels.chars().fold(false, |result, v| c == v || result)
}

fn is_bad_string(text: &str) -> bool {
    let bad_strings = ["ab", "cd", "pq", "xy"];
    bad_strings
        .iter()
        .fold(false, |result, bad_string| result || text.eq(*bad_string))
}

fn is_nice(text: &str) -> bool {
    struct Step {
        previous: char,
        vowel_count: i32,
        double_count: i32,
        bad_string: bool,
    }

    let mut chars = text.chars();
    let first_char = chars.next().unwrap();
    let result = chars.fold(
        Step {
            previous: first_char,
            vowel_count: if is_vowel(first_char) { 1 } else { 0 },
            double_count: 0,
            bad_string: false,
        },
        |result, c| {
            let t: String = [result.previous, c].iter().collect();
            Step {
                previous: c,
                vowel_count: result.vowel_count + if is_vowel(c) { 1 } else { 0 },
                double_count: result.double_count + if result.previous == c { 1 } else { 0 },
                bad_string: result.bad_string || is_bad_string(&t.to_string()),
            }
        },
    );
    !result.bad_string && result.double_count >= 1 && result.vowel_count >= 3
}

fn contains_double_char2(text: &str) -> bool {
    let mut chars = text.chars();
    let first = chars.next().unwrap();
    let second = chars.next().unwrap();
    let result = chars.fold((0, first, second), |(count, first, second), current| {
        (
            count + if first == current { 1 } else { 0 },
            second,
            current,
        )
    });
    result.0 > 0
}

fn detrip(text: &str) -> String {
    let mut chars = text.chars();
    let count = text.len();
    let mut result: Vec<char> = Vec::new();
    result.push(chars.next().unwrap());
    result.push(chars.next().unwrap());
    result.push(chars.next().unwrap());

    for (i, c) in chars.enumerate() {
        let last3 = vec![
            result[result.len() - 3],
            result[result.len() - 2],
            result[result.len() - 1],
        ];

        if last3[0] == last3[1] && last3[1] == last3[2] && last3[2] != c {
            result.pop();
        }

        if i + 3 == count - 1 && last3[1] == last3[2] && last3[2] == c {
            continue;
        }

        result.push(c);
    }

    result.iter().collect::<String>()
}

fn contains_repeating_pair2(text: &str) -> bool {
    let detrip_text = detrip(text);
    let pairs = detrip_text
        .chars()
        .tuple_windows::<(char, char)>()
        .map(|(a, b)| [a, b].iter().collect::<String>());
    let mut counts: HashMap<String, i32> = HashMap::new();

    for pair in pairs {
        counts
            .entry(pair.clone())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    counts.values().filter(|value| **value > 1).count() > 0
}

fn is_nice2(text: &str) -> bool {
    contains_double_char2(text) && contains_repeating_pair2(text)
}

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();
    let nice_count = text
        .lines()
        .fold(0, |result, line| result + if is_nice(line) { 1 } else { 0 });
    println!("1) {}", nice_count);
    let nice_count2 = text.lines().fold(0, |result, line| {
        result + if is_nice2(line) { 1 } else { 0 }
    });
    println!("2) {}", nice_count2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
    }

    #[test]
    fn test_naughty() {
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_is_vowel() {
        assert!(is_vowel('a'));
        assert!(is_vowel('e'));
        assert!(is_vowel('i'));
        assert!(is_vowel('o'));
        assert!(is_vowel('u'));
        assert!(!is_vowel('z'));
    }

    #[test]
    fn test_is_bad_string() {
        assert!(is_bad_string("ab"));
        assert!(is_bad_string("cd"));
        assert!(is_bad_string("pq"));
        assert!(is_bad_string("xy"));
        assert!(!is_bad_string("aa"));
    }

    #[test]
    fn test_is_nice2() {
        assert!(is_nice2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice2("xxyxx"));
    }

    #[test]
    fn test_naughty2() {
        assert!(!is_nice2("uurcxstgmygtbstg"));
        assert!(!is_nice2("ieodomkazucvgmuy"));
    }

    #[test]
    fn test_contains_double_char2() {
        assert!(contains_double_char2("xyx"));
        assert!(contains_double_char2("abcdefeghi"));
        assert!(contains_double_char2("aaa"));
        assert!(!contains_double_char2("uurcxstgmygtbstg"));
    }

    #[test]
    fn test_contains_repeating_pair2() {
        assert!(contains_repeating_pair2("xyxy"));
        assert!(contains_repeating_pair2("aabcdefgaa"));
        assert!(contains_repeating_pair2("uurcxstgmygtbstg"));
        assert!(!contains_repeating_pair2("ieodomkazucvgmuy"));
    }

    #[test]
    fn test_detrip() {
        assert_eq!(detrip("xaaa"), "xaa");
        assert_eq!(detrip("bbaaabb"), "bbaabb");
        assert_eq!(detrip("xaba"), "xaba");
        assert_eq!(detrip("xbaa"), "xbaa");
        assert_eq!(detrip("aabaa"), "aabaa");
        assert_eq!(detrip("aaaa"), "aaaa");
    }
}
