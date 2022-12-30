use itertools::Itertools;
use regex::Regex;
use std::fs;

type Assignment = (i32, usize, usize);
type Stack = Vec<char>;
type Stacks = Vec<Stack>;

fn rotate(xss: Vec<Vec<Option<char>>>) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();
    let stack_count = xss[0].len();

    for stack_index in 0..stack_count {
        if result.len() < stack_index + 1 {
            result.push(Vec::new());
        }

        for row_index in (0..xss.len()).rev() {
            match xss[row_index][stack_index] {
                None => {}
                Some(c) => {
                    result[stack_index].push(c);
                }
            }
        }
    }

    result
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let stack_count = (input.lines().next().unwrap().len() + 1) / 4;
    let stack_indexes = (1..((stack_count * 4) + 1))
        .step_by(4)
        .collect::<Vec<usize>>();
    let stack_data = input
        .lines()
        .take_while(|line| !line.starts_with(" 1"))
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|(i, _)| stack_indexes.contains(i))
                .map(|(_, c)| match c {
                    ' ' => None,
                    c => Some(c),
                })
                .collect::<Vec<Option<char>>>()
        })
        .collect::<Vec<Vec<Option<char>>>>();

    rotate(stack_data)
}

fn parse_assignments<'a>(input: &'a str) -> impl 'a + Iterator<Item = Assignment> {
    let assignments = input.lines().skip_while(|line| !line.is_empty()).skip(1);
    assignments.map(parse_assignment_line)
}

fn parse_assignment_line(line: &str) -> Assignment {
    let pattern = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let captures = pattern.captures(line).unwrap();
    let amount = captures[1].parse::<i32>().unwrap();
    let from = captures[2].parse::<usize>().unwrap();
    let to = captures[3].parse::<usize>().unwrap();
    (amount, from, to)
}

fn apply_assignment((amount, from, to): Assignment, stacks: &mut Stacks) -> &Stacks {
    for _ in 0..amount {
        let value = stacks[from - 1].pop().unwrap();
        stacks[to - 1].push(value);
    }
    stacks
}

fn apply_assignment2((amount, from, to): Assignment, stacks: &mut Stacks) -> &Stacks {
    let length = stacks[from - 1].len();
    let values: Vec<char> = stacks[from - 1]
        .iter()
        .copied()
        .rev()
        .take(amount as usize)
        .rev()
        .collect();
    stacks[from - 1].truncate(length - (amount as usize));
    for value in values {
        stacks[to - 1].push(value);
    }
    stacks
}

fn part1(input: &str) -> String {
    let mut stacks = parse_stacks(input);
    let assignments = parse_assignments(input);
    for assignment in assignments {
        apply_assignment(assignment, &mut stacks);
    }
    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .into_iter()
        .join("")
}

fn part2(input: &str) -> String {
    let mut stacks = parse_stacks(input);
    let assignments = parse_assignments(input);
    for assignment in assignments {
        apply_assignment2(assignment, &mut stacks);
    }
    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .into_iter()
        .join("")
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("part 1: {:?}", part1(input.as_str()));
    println!("part 2: {:?}", part2(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_stacks() {
        let input = fs::read_to_string("./test-input.txt").unwrap();
        let stacks = parse_stacks(&input);
        assert_eq!(stacks.len(), 3);
        assert_eq!(&stacks[0][..], ['Z', 'N']);
        assert_eq!(&stacks[1][..], ['M', 'C', 'D']);
        assert_eq!(&stacks[2][..], ['P']);
    }

    #[test]
    fn test_parse_assignments() {
        let input = fs::read_to_string("./test-input.txt").unwrap();
        let mut assignments = parse_assignments(&input);
        assert_eq!(assignments.next().unwrap(), (1, 2, 1));
        assert_eq!(assignments.next().unwrap(), (3, 1, 3));
        assert_eq!(assignments.next().unwrap(), (2, 2, 1));
        assert_eq!(assignments.next().unwrap(), (1, 1, 2));
    }

    #[test]
    fn test_parse_assignment_line() {
        assert_eq!(parse_assignment_line("move 1 from 2 to 1"), (1, 2, 1));
        assert_eq!(parse_assignment_line("move 3 from 1 to 3"), (3, 1, 3));
        assert_eq!(parse_assignment_line("move 2 from 2 to 1"), (2, 2, 1));
        assert_eq!(parse_assignment_line("move 1 from 1 to 2"), (1, 1, 2));
    }

    #[test]
    fn test_rotate() {
        assert_eq!(
            rotate(vec![
                vec![None, Some('D'), None],
                vec![Some('N'), Some('C'), None],
                vec![Some('Z'), Some('M'), Some('P')]
            ]),
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
        );
    }

    #[test]
    fn test_apply_assignment() {
        let mut stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let stacks = apply_assignment((2, 1, 3), &mut stacks);
        assert_eq!(
            *stacks,
            vec![vec![], vec!['M', 'C', 'D'], vec!['P', 'N', 'Z']]
        );
    }

    #[test]
    fn test_apply_assignment2() {
        assert_eq!(
            *apply_assignment2(
                (1, 2, 1),
                &mut vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
            ),
            vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']]
        );
        assert_eq!(
            *apply_assignment2(
                (3, 1, 3),
                &mut vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']],
            ),
            vec![vec![], vec!['M', 'C'], vec!['P', 'Z', 'N', 'D']]
        );
    }
}
