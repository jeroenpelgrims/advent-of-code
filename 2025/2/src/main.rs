use core::str;
use std::fs;

type ProductId = u64;
type ProductIdRange = (ProductId, ProductId);

fn parse_input_line(line: &str) -> ProductIdRange {
    let parts: Vec<&str> = line.split('-').collect();
    (parts[0].parse().unwrap(), parts[1].parse().unwrap())
}

fn read_input(path: String) -> Vec<ProductIdRange> {
    let input = fs::read_to_string(path).expect("Failed to read file");
    let lines = input.split(',');
    lines.map(parse_input_line).collect()
}

fn expand_range(range: ProductIdRange) -> Vec<ProductId> {
    let (start, end) = range;
    (start..=end).collect()
}

fn is_valid(id: ProductId) -> bool {
    let str_id = id.to_string();
    if str_id.len() % 2 != 0 {
        return true;
    }
    let mid = str_id.len() / 2;
    let begin = &str_id[0..mid];
    return format!("{}{}", begin, begin) != str_id;
}

fn main() {
    let part1 = read_input("input.txt".to_string())
        .into_iter()
        .map(expand_range)
        .collect::<Vec<_>>()
        .concat()
        .into_iter()
        .filter(|id| !is_valid(*id))
        .map(|id| id as u64)
        .sum::<u64>();
    println!("{:?}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_input_line {
        use super::*;

        #[test]
        fn test_par() {
            let line = "11-22";
            let expected = (11, 22);
            assert_eq!(parse_input_line(line), expected);

            let line = "2121212118-2121212124";
            let expected = (2121212118, 2121212124);
            assert_eq!(parse_input_line(line), expected);
        }
    }

    mod parse_input {
        use super::*;

        #[test]
        fn test_read_input() {
            let input = read_input("test-input.txt".to_string());
            assert_eq!(input.len(), 11);
            assert_eq!(input[0], (11, 22));
            assert_eq!(input[input.len() - 1], (2121212118, 2121212124));
        }
    }

    mod expand_range {
        use super::*;

        #[test]
        fn expands_range_normally() {
            let range = (11, 22);
            let expected = vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22];
            assert_eq!(expand_range(range), expected);
        }

        #[test]
        fn returns_empty_vec_for_negative_range() {
            let range = (22, 11);
            let expected: Vec<ProductId> = vec![];
            assert_eq!(expand_range(range), expected);
        }

        #[test]
        fn returns_single_element_for_single_value_range() {
            let range = (15, 15);
            let expected = vec![15];
            assert_eq!(expand_range(range), expected);
        }
    }

    mod is_valid {
        use super::*;

        #[test]
        fn detects_invalid_correctly() {
            assert!(!is_valid(11));
            assert!(!is_valid(22));
            assert!(!is_valid(99));
            assert!(!is_valid(1010));
            assert!(!is_valid(38593859));
        }

        #[test]
        fn detects_valid_correctly() {
            assert!(is_valid(446443));
        }
    }
}
