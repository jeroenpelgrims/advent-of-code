use std::fs;

fn get_first_last(line: &str) -> (u32, u32) {
    let digits: Vec<u32> = line
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    if digits.len() == 1 {
        let first = digits.first().unwrap().clone();
        (first, first)
    } else {
        let first = digits.first().unwrap().clone();
        let last = digits.last().unwrap().clone();
        (first, last)
    }
}

fn solve1(path: &str) -> u32 {
    let text = fs::read_to_string(path).unwrap();
    text.lines()
        .map(get_first_last)
        .map(|(first, last)| first * 10 + last)
        .sum()
}

fn main() {
    let solution = solve1("./input.txt");
    println!("1: {}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_last() {
        assert_eq!(get_first_last("1abc2"), (1, 2));
        assert_eq!(get_first_last("pqr3stu8vwx"), (3, 8));
        assert_eq!(get_first_last("a1b2c3d4e5f"), (1, 5));
        assert_eq!(get_first_last("treb7uchet"), (7, 7));
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1("./test_input.txt"), 142);
    }
}
