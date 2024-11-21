use core::panic;
use std::collections::HashMap;
use std::fs;

type Context = HashMap<Variable, ValueOrOperation>;
type Variable = String;

#[derive(Debug, PartialEq)]
enum ValueOrOperation {
    Value(i32),
    And(Variable, Variable),
    Or(Variable, Variable),
    Lshift(Variable, i32),
    Rshift(Variable, i32),
    Not(Variable),
}

type Definition = (Variable, ValueOrOperation);

fn parse_value_or_operation(value_or_operation: &str) -> ValueOrOperation {
    let parts = value_or_operation.split(" ").collect::<Vec<&str>>();

    match parts.as_slice() {
        [left, "AND", right] => ValueOrOperation::And(left.to_string(), right.to_string()),
        [left, "OR", right] => ValueOrOperation::Or(left.to_string(), right.to_string()),
        [left, "LSHIFT", right] => {
            ValueOrOperation::Lshift(left.to_string(), right.parse::<i32>().unwrap())
        }
        [left, "RSHIFT", right] => {
            ValueOrOperation::Rshift(left.to_string(), right.parse::<i32>().unwrap())
        }
        ["NOT", variable] => ValueOrOperation::Not(variable.to_string()),
        [value] => ValueOrOperation::Value(value.parse::<i32>().unwrap()),
        _ => panic!("Unsupported pattern for value or operation"),
    }
}

fn parse_line(line: &str) -> Definition {
    let parts = line.split(" -> ").collect::<Vec<&str>>();

    match parts.as_slice() {
        [value_or_operation, variable] => {
            let parsed = parse_value_or_operation(value_or_operation);
            (variable.to_string(), parsed)
        }
        _ => panic!("Unsupported definition line"),
    }
}

fn make_context(input: &str) -> Context {
    let definitions = input.lines().map(parse_line).collect::<Vec<_>>();
    let mut context: Context = HashMap::new();
    for (variable, variable_or_operation) in definitions {
        context.insert(variable, variable_or_operation);
    }
    context
}

fn eval(variable_or_operation: ValueOrOperation, context: Context) {
    todo!()
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let context = make_context(&input);
    println!("{:?}", context);
    // println!("{:?}", test_input);
    // let text = fs::read_to_string("./input.txt").unwrap();
    // let instructions = text.lines().map(line_to_instruction).collect::<Vec<_>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value_or_operation() {
        assert_eq!(
            parse_value_or_operation("123"),
            ValueOrOperation::Value(123)
        );
        assert_eq!(
            parse_value_or_operation("x AND y"),
            ValueOrOperation::And("x".to_string(), "y".to_string())
        );
        assert_eq!(
            parse_value_or_operation("x OR y"),
            ValueOrOperation::Or("x".to_string(), "y".to_string())
        );
        assert_eq!(
            parse_value_or_operation("x LSHIFT 2"),
            ValueOrOperation::Lshift("x".to_string(), 2)
        );
        assert_eq!(
            parse_value_or_operation("y RSHIFT 2"),
            ValueOrOperation::Rshift("y".to_string(), 2)
        );
        assert_eq!(
            parse_value_or_operation("NOT y"),
            ValueOrOperation::Not("y".to_string())
        );
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("456 -> y"),
            ("y".to_string(), ValueOrOperation::Value(456))
        );
        assert_eq!(
            parse_line("x AND y -> d"),
            (
                "d".to_string(),
                ValueOrOperation::And("x".to_string(), "y".to_string())
            )
        );
        assert_eq!(
            parse_line("x OR y -> e"),
            (
                "e".to_string(),
                ValueOrOperation::Or("x".to_string(), "y".to_string())
            )
        );
        assert_eq!(
            parse_line("x LSHIFT 2 -> f"),
            (
                "f".to_string(),
                ValueOrOperation::Lshift("x".to_string(), 2)
            )
        );
        assert_eq!(
            parse_line("y RSHIFT 2 -> g"),
            (
                "g".to_string(),
                ValueOrOperation::Rshift("y".to_string(), 2)
            )
        );
        assert_eq!(
            parse_line("NOT y -> i"),
            ("i".to_string(), ValueOrOperation::Not("y".to_string()))
        );
        assert_eq!(
            parse_line("123 -> i"),
            ("i".to_string(), ValueOrOperation::Value(123))
        );
    }

    #[test]
    fn test_make_context() {
        let test_input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

        let expected: Context = vec![
            ("x".to_string(), ValueOrOperation::Value(123)),
            ("y".to_string(), ValueOrOperation::Value(456)),
            (
                "d".to_string(),
                ValueOrOperation::And("x".to_string(), "y".to_string()),
            ),
            (
                "e".to_string(),
                ValueOrOperation::Or("x".to_string(), "y".to_string()),
            ),
            (
                "f".to_string(),
                ValueOrOperation::Lshift("x".to_string(), 2),
            ),
            (
                "g".to_string(),
                ValueOrOperation::Rshift("y".to_string(), 2),
            ),
            ("h".to_string(), ValueOrOperation::Not("x".to_string())),
            ("i".to_string(), ValueOrOperation::Not("y".to_string())),
        ]
        .into_iter()
        .collect();
        let actual = make_context(test_input);
        assert_eq!(actual, expected);
    }
}
