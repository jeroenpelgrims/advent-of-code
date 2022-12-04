use itertools::Itertools;
use std::{fs, iter::Iterator};

fn parse_backpack(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn find_shared_item((compartment1, compartment2): (&str, &str)) -> char {
    compartment1
        .chars()
        .cartesian_product(compartment2.chars())
        .find(|(x, y)| x == y)
        .unwrap()
        .0
}

fn priority(c: char) -> i32 {
    let lower = ('a'..='z').zip(1..=26);
    let upper = ('A'..='Z').zip(27..=52);
    let (_, priority) = lower.chain(upper).find(|(c_, _)| c == *c_).unwrap();
    priority
}

fn backpack_priorities<'a>(backpacks: impl Iterator<Item = (&'a str, &'a str)>) -> i32 {
    backpacks.map(find_shared_item).map(priority).sum::<i32>()
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let backpacks = input.lines().map(parse_backpack);
    println!("1: {:?}", backpack_priorities(backpacks));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_backpack() {
        assert_eq!(
            parse_backpack("vJrwpWtwJgWrhcsFMMfFFhFp"),
            ("vJrwpWtwJgWr", "hcsFMMfFFhFp")
        );
        assert_eq!(
            parse_backpack("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL")
        );
        assert_eq!(
            parse_backpack("PmmdzqPrVvPwwTWBwg"),
            ("PmmdzqPrV", "vPwwTWBwg")
        );
        assert_eq!(
            parse_backpack("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            ("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn")
        );
        assert_eq!(parse_backpack("ttgJtRGJQctTZtZT"), ("ttgJtRGJ", "QctTZtZT"));
        assert_eq!(
            parse_backpack("CrZsJsPPZsGzwwsLwLmpwMDw"),
            ("CrZsJsPPZsGz", "wwsLwLmpwMDw")
        );
    }

    #[test]
    fn test_find_shared_item() {
        assert_eq!(
            find_shared_item(parse_backpack("vJrwpWtwJgWrhcsFMMfFFhFp")),
            'p'
        );
        assert_eq!(
            find_shared_item(parse_backpack("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")),
            'L'
        );
        assert_eq!(find_shared_item(parse_backpack("PmmdzqPrVvPwwTWBwg")), 'P');
        assert_eq!(
            find_shared_item(parse_backpack("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn")),
            'v'
        );
        assert_eq!(find_shared_item(parse_backpack("ttgJtRGJQctTZtZT")), 't');
        assert_eq!(
            find_shared_item(parse_backpack("CrZsJsPPZsGzwwsLwLmpwMDw")),
            's'
        );
    }

    #[test]
    fn test_priority() {
        assert_eq!(priority('p'), 16);
        assert_eq!(priority('L'), 38);
        assert_eq!(priority('P'), 42);
        assert_eq!(priority('v'), 22);
        assert_eq!(priority('t'), 20);
        assert_eq!(priority('s'), 19);
    }

    #[test]
    fn test_backpack_priorities() {
        let input = fs::read_to_string("./test-input.txt").unwrap();
        let backpacks = input.lines().map(parse_backpack);
        assert_eq!(backpack_priorities(backpacks), 157);
    }
}
