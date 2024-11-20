use core::panic;
use std::fs;

type Point = (i32, i32);

#[derive(Copy, Clone)]
struct Area {
    from: Point,
    until: Point,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum StateAction {
    On,
    Off,
    Toggle,
}

struct Instruction {
    action: StateAction,
    area: Area,
}

#[derive(Debug)]
struct Light {
    position: Point,
    state: bool,
}

fn is_point_in_area((x, y): Point, area: Area) -> bool {
    let (x1, y1) = area.from;
    let (x2, y2) = area.until;
    let x_min = x1.min(x2);
    let x_max = x1.max(x2);
    let y_min = y1.min(y2);
    let y_max = y1.max(y2);

    x_min <= x && x <= x_max && y_min <= y && y <= y_max
}

fn point_from_str(str: &str) -> Point {
    let vec = str.split(",").collect::<Vec<&str>>();
    let [from_str, until_str] = vec.as_slice() else {
        panic!()
    };
    let from = from_str.parse::<i32>().unwrap();
    let until = until_str.parse::<i32>().unwrap();

    (from, until)
}

fn line_to_instruction(line: &str) -> Instruction {
    let line_vec = line.split(" ").collect::<Vec<&str>>();
    let parts = line_vec.as_slice();
    match parts {
        ["turn", "on", from, _, until] => Instruction {
            action: StateAction::On,
            area: Area {
                from: point_from_str(from),
                until: point_from_str(until),
            },
        },
        ["turn", "off", from, _, until] => Instruction {
            action: StateAction::Off,
            area: Area {
                from: point_from_str(from),
                until: point_from_str(until),
            },
        },
        ["toggle", from, _, until] => Instruction {
            action: StateAction::Toggle,
            area: Area {
                from: point_from_str(from),
                until: point_from_str(until),
            },
        },
        _ => panic!("Unsupported instruction pattern"),
    }
}

fn apply_instruction(state: bool, action: StateAction) -> bool {
    match action {
        StateAction::On => true,
        StateAction::Off => false,
        StateAction::Toggle => !state,
    }
}

fn apply_instructions(position: Point, instructions: &Vec<Instruction>) -> bool {
    instructions.iter().fold(false, |state, instruction| {
        if is_point_in_area(position, instruction.area) {
            apply_instruction(state, instruction.action)
        } else {
            state
        }
    })
}

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();
    let instructions = text.lines().map(line_to_instruction).collect::<Vec<_>>();

    let columns = 1000;
    let positions = (0..999_999).map(|i| {
        let row = i / columns;
        let column = i % columns;
        (column, row)
    });
    let answer1 = positions
        .map(|position| apply_instructions(position, &instructions))
        .fold(0, |result, state| result + if state { 1 } else { 0 });
    println!("1: {:?}", answer1);

    // let lights: Vec<_> = (0..999)
    //     .flat_map(|i| {
    //         (0..999).map(move |j| Light {
    //             position: (i, j),
    //             state: false,
    //         })
    //     })
    //     .collect();
    // print!("{:?}", lights);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_to_instruction_on() {
        let line = line_to_instruction("turn on 489,959 through 759,964");
        assert_eq!(line.action, StateAction::On);
        assert_eq!(line.area.from, (489, 959));
        assert_eq!(line.area.until, (759, 964));
    }

    #[test]
    fn test_line_to_instruction_off() {
        let line = line_to_instruction("turn off 820,516 through 871,914");
        assert_eq!(line.action, StateAction::Off);
        assert_eq!(line.area.from, (820, 516));
        assert_eq!(line.area.until, (871, 914));
    }

    #[test]
    fn test_line_to_instruction_toggle() {
        let line = line_to_instruction("toggle 756,965 through 812,992");
        assert_eq!(line.action, StateAction::Toggle);
        assert_eq!(line.area.from, (756, 965));
        assert_eq!(line.area.until, (812, 992));
    }

    #[test]
    fn test_point_from_str() {
        let p = point_from_str("489,959");
        assert_eq!(p, (489, 959));
    }

    #[test]
    fn test_is_point_in_area() {
        assert!(is_point_in_area(
            (5, 5),
            Area {
                from: (0, 0),
                until: (10, 10),
            }
        ));
        assert!(!is_point_in_area(
            (0, 5),
            Area {
                from: (1, 0),
                until: (10, 10),
            }
        ));
        assert!(!is_point_in_area(
            (5, 0),
            Area {
                from: (0, 1),
                until: (10, 10),
            }
        ));
    }

    #[test]
    fn test_apply_instruction() {
        assert!(apply_instruction(false, StateAction::On));
        assert!(!apply_instruction(true, StateAction::Off));
        assert!(!apply_instruction(true, StateAction::Toggle));
        assert!(apply_instruction(false, StateAction::Toggle));
    }

    #[test]
    fn test_apply_instructions() {
        // todo!()
    }
}
