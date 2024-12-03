use core::panic;
use std::{fs, iter::once};

use regex::Regex;

#[derive(Debug, Clone)]
enum Operation {
    Mul(i32, i32),
    Do,
    Dont,
}

fn find_operations(input: &str) -> Vec<Operation> {
    let re = Regex::new(r"(mul|do|don't)\(((\d+),(\d+))?\)").unwrap();
    let matches: Vec<Operation> = re
        .captures_iter(input)
        .map(|m| {
            let func = &m[1];
            match func {
                "mul" => {
                    let x: i32 = m[3].parse().unwrap();
                    let y: i32 = m[4].parse().unwrap();
                    Operation::Mul(x, y)
                }
                "do" => Operation::Do,
                "don't" => Operation::Dont,
                _ => panic!("Unsupported operation found: {:?}", &m[0]),
            }
        })
        .collect();
    matches
}

fn drop_donts(xs: &Vec<Operation>, dropping: bool) -> Vec<Operation> {
    match &xs[..] {
        [Operation::Dont, rest @ ..] => drop_donts(&rest.to_vec(), true),
        [Operation::Do, rest @ ..] => drop_donts(&rest.to_vec(), false),
        [op @ Operation::Mul(_, _), rest @ ..] => {
            if dropping {
                drop_donts(&rest.to_vec(), dropping)
            } else {
                let rest_dropped = drop_donts(&rest.to_vec(), dropping);
                once(op.clone()).chain(rest_dropped).collect()
            }
        }
        [] => vec![],
    }
}

fn apply(operations: &Vec<Operation>) -> i32 {
    operations
        .iter()
        .fold(0, |total, operation| match operation {
            Operation::Mul(x, y) => total + x * y,
            _ => total,
        })
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let operations = find_operations(&input);

    println!("1: {:?}", apply(&operations));
    println!("2: {:?}", apply(&drop_donts(&operations, false)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe() {}
}
