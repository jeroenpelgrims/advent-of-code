use std::{fs, iter::zip};

fn parse_line(line: &str) -> (i32, i32) {
    let elems = line.split("   ").collect::<Vec<&str>>();
    let [a, b] = elems.as_slice() else {
        panic!("Unsupported line format: {}", line)
    };
    (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
}

fn tuples_to_lists(tuples: Vec<(i32, i32)>) -> (Vec<i32>, Vec<i32>) {
    let (mut a, mut b): (Vec<i32>, Vec<i32>) = tuples.into_iter().unzip();
    a.sort_unstable();
    b.sort_unstable();
    (a, b)
}

fn total_distance(a: Vec<i32>, b: Vec<i32>) -> i32 {
    zip(a, b).fold(0, |result, (a, b)| result + (a - b).abs())
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let items: Vec<(i32, i32)> = input.lines().map(parse_line).collect();
    let (a, b) = tuples_to_lists(items);
    println!("1: {}", total_distance(a, b));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let parsed = parse_line("1   2");
        assert_eq!(parsed, (1, 2));
    }

    #[test]
    fn test_tuples_to_lists() {
        let (a, b) = tuples_to_lists(vec![(5, 2), (3, 6), (1, 4)]);
        assert_eq!(a, vec![1, 3, 5]);
        assert_eq!(b, vec![2, 4, 6]);
    }
}
