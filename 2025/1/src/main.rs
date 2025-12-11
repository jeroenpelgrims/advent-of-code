use std::{fs, result, vec};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Rotation {
    Left(i32),
    Right(i32),
}

fn parse_input_line(line: &str) -> Rotation {
    let (direction, amount_str) = line.split_at(1);
    let amount: i32 = amount_str.parse().unwrap();

    match direction {
        "L" => Rotation::Left(amount),
        "R" => Rotation::Right(amount),
        _ => panic!("Invalid rotation direction"),
    }
}

fn parse_input(input: String) -> Vec<Rotation> {
    input.lines().map(parse_input_line).collect()
}

fn apply_rotation(rotation: Rotation, value: i32) -> i32 {
    match rotation {
        Rotation::Left(amount) => (value + (100 - (amount % 100))) % 100,
        Rotation::Right(amount) => (value + amount) % 100,
    }
}

fn apply_rotations(rotations: Vec<Rotation>, start_value: i32) -> Vec<i32> {
    let results =
        rotations
            .into_iter()
            .fold(vec![start_value], |results, rot| {
                let head = &results[0];
                let rest = &results[1..];
                let next = apply_rotation(rot, *head);
                vec![vec![next], vec![*head], rest.to_vec()].concat()
                // vec![next, *head].extend_from_slice(rest)
            });
    results.into_iter().rev().collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = parse_input(input);
    let part1 = apply_rotations(input, 50)
        .iter()
        .fold(0, |acc, &x| if x == 0 { acc + 1 } else { acc });

    println!("Part 1: {}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_input_line {
        use super::*;

        #[test]
        fn should_parse_left_rotation() {
            let result = parse_input_line("L90");
            assert_eq!(result, Rotation::Left(90));
        }

        #[test]
        fn should_parse_right_rotation() {
            let result = parse_input_line("R90");
            assert_eq!(result, Rotation::Right(90));
        }

        #[test]
        fn should_parse_zero_rotation() {
            let result_left = parse_input_line("L0");
            assert_eq!(result_left, Rotation::Left(0));

            let result_right = parse_input_line("R0");
            assert_eq!(result_right, Rotation::Right(0));
        }

        #[test]
        fn should_parse_large_number_rotation() {
            let result_left = parse_input_line("L450");
            assert_eq!(result_left, Rotation::Left(450));

            let result_right = parse_input_line("R720");
            assert_eq!(result_right, Rotation::Right(720));
        }
    }

    mod parse_input {
        use super::*;

        #[test]
        fn should_parse_multiple_rotations() {
            let input = "L90\nR180\nL270";
            let result = parse_input(input.to_string());
            let expected = vec![
                Rotation::Left(90),
                Rotation::Right(180),
                Rotation::Left(270),
            ];
            assert_eq!(result, expected);
        }

        #[test]
        fn should_parse_test_input() {
            let input = fs::read_to_string("./test-input.txt").unwrap();
            let input = parse_input(input);
            let expected = vec![
                Rotation::Left(68),
                Rotation::Left(30),
                Rotation::Right(48),
                Rotation::Left(5),
                Rotation::Right(60),
                Rotation::Left(55),
                Rotation::Left(1),
                Rotation::Left(99),
                Rotation::Right(14),
                Rotation::Left(82),
            ];
            assert_eq!(input, expected);
        }
    }

    mod apply_rotation {
        use super::*;

        #[test]
        fn should_apply_left_rotation() {
            let result = apply_rotation(Rotation::Left(10), 50);
            assert_eq!(result, 40);
        }

        #[test]
        fn should_apply_right_rotation() {
            let result = apply_rotation(Rotation::Right(10), 50);
            assert_eq!(result, 60);
        }

        #[test]
        fn should_wrap_around_on_left_rotation() {
            let result = apply_rotation(Rotation::Left(1), 0);
            assert_eq!(result, 99);
        }

        #[test]
        fn should_wrap_around_on_right_rotation() {
            let result = apply_rotation(Rotation::Right(1), 99);
            assert_eq!(result, 0);
        }
    }

    mod apply_rotations {
        use super::*;

        #[test]
        fn should_apply_multiple_rotations() {
            let input = fs::read_to_string("./test-input.txt").unwrap();
            let input = parse_input(input);
            let result = apply_rotations(input, 50);
            assert_eq!(result, vec![50, 82, 52, 0, 95, 55, 0, 99, 0, 14, 32]);
        }
    }
}
