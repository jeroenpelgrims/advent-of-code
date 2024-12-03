use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Operation {
    func: String,
    x: i32,
    y: i32,
}

fn find_operations(input: &str) -> Vec<Operation> {
    let re = Regex::new(r"(mul)\((\d+),(\d+)\)").unwrap();
    let matches: Vec<Operation> = re
        .captures_iter(input)
        .map(|m| {
            let func = m[1].to_string();
            let x: i32 = m[2].parse().unwrap();
            let y: i32 = m[3].parse().unwrap();
            Operation {
                func: func,
                x: x,
                y: y,
            }
        })
        .collect();
    matches
}

fn apply(operations: Vec<Operation>) -> i32 {
    operations.iter().fold(0, |total, operation| {
        match operation.func.as_str() {
            "mul" => total + operation.x * operation.y,
            _ => panic!("Unsupported operation: {:?}", operation),
        }
    })
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let operations = find_operations(&input);
    println!("1: {:?}", apply(operations));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe() {}
}
