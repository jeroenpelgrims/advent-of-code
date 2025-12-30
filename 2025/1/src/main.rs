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

fn apply_rotation(rot: &Rotation, start_value: i32) -> (i32, i32) {
    match rot {
        Rotation::Left(amount) => {
            let overflows = amount / 100 as i32;
            let rest = amount % 100;
            let overflow = if start_value - rest <= 0 && start_value != 0 {
                1
            } else {
                0
            };
            let next = (start_value - rest).rem_euclid(100);
            (next, overflows + overflow)
        }
        Rotation::Right(amount) => {
            let overflows = amount / 100 as i32;
            let rest = amount % 100;
            let overflow = if start_value + rest > 99 && start_value != 0 {
                1
            } else {
                0
            };
            let next = (start_value + rest) % 100;
            (next, overflows + overflow)
        }
    }
}

fn apply_rotations(
    rotations: &Vec<Rotation>,
    start_value: i32,
) -> Vec<(i32, i32)> {
    let values = rotations.into_iter().fold(vec![], |results, rot| {
        let previous = results.last().unwrap_or(&(start_value, 0)).clone();
        let (start_value, _) = previous;
        let next_val = apply_rotation(rot, start_value);
        vec![results, vec![next_val]].concat()
    });
    values
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = parse_input(input);
    let values = apply_rotations(&input, 50);
    let part1 = &values
        .iter()
        .fold(0, |acc, &(value, _)| if value == 0 { acc + 1 } else { acc });
    let part2: i32 = values.iter().map(|(_, overflows)| overflows).sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
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
            let result = apply_rotation(&Rotation::Left(10), 50);
            assert_eq!(result, (40, 0));
        }

        #[test]
        fn should_apply_right_rotation() {
            let result = apply_rotation(&Rotation::Right(10), 50);
            assert_eq!(result, (60, 0));
        }

        #[test]
        fn should_wrap_around_on_left_rotation() {
            let result = apply_rotation(&Rotation::Left(1), 0);
            assert_eq!(result, (99, 0));
        }

        #[test]
        fn should_wrap_around_on_right_rotation() {
            let result = apply_rotation(&Rotation::Right(1), 99);
            assert_eq!(result, (0, 1));
        }

        #[test]
        fn detects_simple_overflow() {
            let result = apply_rotation(&Rotation::Left(60), 50);
            assert_eq!(result, (90, 1));
            let result = apply_rotation(&Rotation::Right(60), 50);
            assert_eq!(result, (10, 1));
        }

        #[test]
        fn detects_multiple_overflows() {
            let result = apply_rotation(&Rotation::Left(260), 50);
            assert_eq!(result, (90, 3));
            let result = apply_rotation(&Rotation::Right(260), 50);
            assert_eq!(result, (10, 3));
        }

        #[test]
        fn works_with_example_input() {
            let input = fs::read_to_string("./test-input.txt").unwrap();
            let input = parse_input(input);
            let values = apply_rotations(&input, 50);
            let overflows: i32 =
                values.iter().map(|(_, overflow)| overflow).sum();

            assert_eq!(overflows, 6);
        }

        #[test]
        fn overflow_from_start_value_zero_is_not_overflow() {
            let result = apply_rotation(&Rotation::Left(1), 0);
            assert_eq!(result, (99, 0));
            let result = apply_rotation(&Rotation::Right(1), 0);
            assert_eq!(result, (1, 0));
        }

        #[test]
        fn many_overflows() {
            let result = apply_rotation(&Rotation::Right(1000), 50);
            assert_eq!(result, (50, 10));
        }

        #[test]
        fn many_overflows_but_ends_on_0() {
            let result = apply_rotation(&Rotation::Right(1000), 0);
            assert_eq!(result, (0, 10));
            let result = apply_rotation(&Rotation::Left(1000), 0);
            assert_eq!(result, (0, 10));
        }
        #[test]
        fn extra_cases() {
            assert_eq!(apply_rotation(&Rotation::Right(1), 99), (0, 1));
            assert_eq!(apply_rotation(&Rotation::Left(2), 1), (99, 1));
            assert_eq!(apply_rotation(&Rotation::Right(49), 50), (99, 0));
            assert_eq!(apply_rotation(&Rotation::Left(49), 50), (1, 0));
        }
    }

    mod apply_rotations {
        use super::*;

        #[test]
        fn should_apply_multiple_rotations() {
            let input = fs::read_to_string("./test-input.txt").unwrap();
            let input = parse_input(input);
            let result = apply_rotations(&input, 50);
            assert_eq!(
                result,
                vec![
                    (82, 1),
                    (52, 0),
                    (0, 1),
                    (95, 0),
                    (55, 1),
                    (0, 1),
                    (99, 0),
                    (0, 1),
                    (14, 0),
                    (32, 1)
                ]
            );
        }
    }
}
