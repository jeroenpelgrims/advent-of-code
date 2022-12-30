use std::fs;

use itertools::Itertools;

fn all_different(input: &str) -> bool {
    input.chars().unique().count() == input.len()
}

fn marker(data: &str) -> i32 {
    let result = (3..data.len()).fold(None, |result, i| match result {
        None => {
            let slice = &data[i - 3..i + 1];
            if all_different(slice) {
                Some(i)
            } else {
                None
            }
        }
        _ => result,
    });
    result.unwrap() as i32 + 1
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    print!("part 1: {}", marker(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_marker() {
        let input = fs::read_to_string("./test-input.txt").unwrap();
        let pos = marker(input.as_str());
        assert_eq!(pos, 7);
    }

    #[test]
    fn test_different() {
        assert!(!all_different("mjqj"));
        assert!(all_different("jpqm"));
    }
}
