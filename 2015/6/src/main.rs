type Point = (i32, i32);

struct Area {
    from: Point,
    until: Point,
}

#[derive(PartialEq, Debug)]
enum StateAction {
    On,
    Off,
    Toggle,
}

struct Instruction {
    action: StateAction,
    area: Area,
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

use core::panic;

fn main() {}

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
}
