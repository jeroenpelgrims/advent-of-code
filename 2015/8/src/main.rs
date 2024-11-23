use std::fs;

fn hex_to_char(a: &char, b: &char) -> char {
    let str = vec![*a, *b].to_vec().iter().collect::<String>();
    let hex = u32::from_str_radix(&str, 16).unwrap();
    hex as u8 as char
}

fn unescape(input: &str) -> String {
    fn inner(input: Vec<char>, pattern: Vec<char>) -> Vec<char> {
        if input.len() == 0 {
            return input;
        }

        let (fst, rest) = input.split_first().unwrap();
        let next_pattern = [pattern, vec![*fst]].concat();
        match next_pattern.as_slice() {
            ['\\', '\\'] => [vec!['\\'], inner(rest.to_vec(), vec![])].concat(),
            ['\\', '\"'] => [vec!['"'], inner(rest.to_vec(), vec![])].concat(),
            ['\\', 'x', a, b] => {
                [vec![hex_to_char(a, b)], inner(rest.to_vec(), vec![])].concat()
            }
            ['\\'] | ['\\', 'x'] | ['\\', 'x', _] => {
                inner(rest.to_vec(), next_pattern)
            }
            _ => [vec![*fst], inner(rest.to_vec(), vec![])].concat(),
        }
    }

    let lstripped = input.strip_prefix("\"").unwrap_or(input);
    let no_quotes = lstripped.strip_suffix("\"").unwrap_or(input);
    let chars = no_quotes.chars().collect::<Vec<char>>();
    let unescaped = inner(chars, vec![]);
    unescaped.into_iter().collect()
}

fn answer1(input: &str) -> usize {
    let code_lens = input
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>().len())
        .fold(0, |acc, x| acc + x);
    let str_lens = input
        .split("\n")
        .map(unescape)
        .map(|line| line.chars().collect::<Vec<char>>().len())
        .fold(0, |acc, x| acc + x);
    return code_lens - str_lens;
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let part1 = answer1(&input);
    println!("1: {:?}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unescape_quotes() {
        assert_eq!("", unescape(r#""""#));
    }

    #[test]
    fn test_unescape_regular_text() {
        assert_eq!("abc", unescape(r#""abc""#));
    }

    #[test]
    fn test_unescape_single_char_escape() {
        assert_eq!(r#"\"#, unescape(r#""\\""#));
        assert_eq!("\"", unescape(r#""\"""#));
        assert_eq!("aaa\"aaa", unescape(r#""aaa\"aaa""#));
    }

    #[test]
    fn test_unescape_ascii() {
        assert_eq!("'", unescape(r#"\x27"#));
    }

    #[test]
    fn test_input_1() {
        let unescaped = unescape(r#""njro\x68qgbx\xe4af\"\\suan""#);
        assert_eq!(r#"njrohqgbxäaf"\suan"#, unescaped);
        assert_eq!(1, "ä".len());
    }
}
