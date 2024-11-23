use std::{fmt::format, fs};

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

fn escape(input: &str) -> String {
    let content: String = input
        .chars()
        .flat_map(|c| match c {
            '\\' => vec!['\\', '\\'],
            '"' => vec!['\\', '"'],
            c => vec![c],
        })
        .collect();
    format!("\"{}\"", content)
}

fn number_of_chars(input: &str, to_apply: fn(&str) -> String) -> usize {
    input
        .split("\n")
        .map(to_apply)
        .map(|line| line.chars().collect::<Vec<char>>().len())
        .fold(0, |acc, x| acc + x)
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let part1 = number_of_chars(&input, |x| x.to_string())
        - number_of_chars(&input, unescape);
    println!("1: {:?}", part1);

    let part2 = number_of_chars(&input, escape)
        - number_of_chars(&input, |x| x.to_string());
    println!("2: {:?}", part2);
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
    fn test_encode() {
        assert_eq!(r#""\"\"""#, escape(r#""""#));
    }
}
