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

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();
    let lines = text.lines();
    let nice_count = lines.fold(0, |result, line| result + if is_nice(line) { 1 } else { 0 });
    println!("1) {}", nice_count);
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
    fn test_not_nice() {
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
    fn test_foo() {
        assert!(['c', 'a'].eq(&['c', 'a']));
    }
}
