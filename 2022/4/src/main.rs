use std::fs;

use itertools::Itertools;

type Zone = (i32, i32);
type Assignment = (Zone, Zone);

fn parse_assignment(line: &str) -> Assignment {
    let (zone1, zone2) = line.split_once(',').unwrap();
    let (a, b) = zone1.split_once('-').unwrap();
    let (c, d) = zone2.split_once('-').unwrap();
    (
        (a.parse().unwrap(), b.parse().unwrap()),
        (c.parse().unwrap(), d.parse().unwrap()),
    )
}

fn fully_contains((zone1, zone2): Assignment) -> bool {
    contains(zone1, zone2) || contains(zone2, zone1)
}

fn contains((a, b): Zone, (c, d): Zone) -> bool {
    a <= c && b >= d
}

fn overlaps(((a, b), (c, d)): Assignment) -> bool {
    let sections = (a..=b).chain(c..=d);
    sections.clone().count() > sections.clone().sorted().dedup().count()
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let assignments = input.lines().map(parse_assignment);

    let contains_count = assignments.clone().filter(|x| fully_contains(*x)).count();
    println!("1: {:?}", contains_count);

    let overlapping_count = assignments.clone().filter(|x| overlaps(*x)).count();
    println!("2: {:?}", overlapping_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_assignment() {
        assert_eq!(parse_assignment("2-4,6-8"), ((2, 4), (6, 8)));
        assert_eq!(parse_assignment("2-3,4-5"), ((2, 3), (4, 5)));
        assert_eq!(parse_assignment("5-7,7-9"), ((5, 7), (7, 9)));
        assert_eq!(parse_assignment("2-8,3-7"), ((2, 8), (3, 7)));
        assert_eq!(parse_assignment("6-6,4-6"), ((6, 6), (4, 6)));
        assert_eq!(parse_assignment("2-6,4-8"), ((2, 6), (4, 8)));
    }

    #[test]
    fn test_fully_contains() {
        assert_eq!(fully_contains(((2, 4), (6, 8))), false);
        assert_eq!(fully_contains(((2, 3), (4, 5))), false);
        assert_eq!(fully_contains(((5, 7), (7, 9))), false);
        assert_eq!(fully_contains(((2, 8), (3, 7))), true);
        assert_eq!(fully_contains(((6, 6), (4, 6))), true);
        assert_eq!(fully_contains(((2, 6), (4, 8))), false);
    }

    #[test]
    fn test_contains() {
        assert_eq!(contains((2, 8), (3, 7)), true);
        assert_eq!(contains((6, 6), (4, 6)), false);
    }

    #[test]
    fn test_overlaps() {
        assert_eq!(overlaps(((2, 4), (6, 8))), false);
        assert_eq!(overlaps(((2, 3), (4, 5))), false);
        assert_eq!(overlaps(((5, 7), (7, 9))), true);
        assert_eq!(overlaps(((2, 8), (3, 7))), true);
        assert_eq!(overlaps(((6, 6), (4, 6))), true);
        assert_eq!(overlaps(((2, 6), (4, 8))), true);
    }
}
