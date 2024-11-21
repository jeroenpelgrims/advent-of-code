use core::panic;
use std::borrow::Borrow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;

type Context = HashMap<String, Expression>;

#[derive(Eq, Hash, Debug, PartialEq, Clone)]
enum Term {
    Variable(String),
    Value(u16),
}

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    And(Term, Term),
    Or(Term, Term),
    Lshift(Term, Term),
    Rshift(Term, Term),
    Not(Term),
}

#[derive(Debug, PartialEq, Clone)]
enum Expression {
    Term(Term),
    Operation(Operation),
}

fn parse_term(term: &str) -> Term {
    if let Ok(number) = term.parse::<u16>() {
        Term::Value(number)
    } else {
        Term::Variable(term.to_string())
    }
}

fn parse_operation(operation: &str) -> Operation {
    let parts = operation.split(" ").collect::<Vec<&str>>();

    match parts.as_slice() {
        [left, "AND", right] => {
            Operation::And(parse_term(left), parse_term(right))
        }
        [left, "OR", right] => {
            Operation::Or(parse_term(left), parse_term(right))
        }
        [left, "LSHIFT", right] => {
            Operation::Lshift(parse_term(left), parse_term(right))
        }
        [left, "RSHIFT", right] => {
            Operation::Rshift(parse_term(left), parse_term(right))
        }
        ["NOT", term] => Operation::Not(parse_term(term)),
        _ => panic!("Unsupported pattern for operation"),
    }
}

fn parse_expression(expression: &str) -> Expression {
    let spaces = expression.chars().filter(|&c| c == ' ').count();

    if spaces == 0 {
        Expression::Term(parse_term(expression))
    } else if spaces >= 1 && spaces <= 2 {
        Expression::Operation(parse_operation(expression))
    } else {
        panic!("Unsupported pattern for value or operation");
    }
}

fn parse_line(line: &str) -> (String, Expression) {
    let parts = line.split(" -> ").collect::<Vec<&str>>();

    match parts.as_slice() {
        [expression, variable] => {
            let variable = variable.to_string();
            (variable, parse_expression(expression))
        }
        _ => panic!("Unsupported definition line"),
    }
}

fn make_context(input: &str) -> Context {
    let definitions = input.lines().map(parse_line).collect::<Vec<_>>();
    let mut context: Context = HashMap::new();
    for (term, expression) in definitions {
        context.insert(term, expression);
    }
    context
}

fn eval_operation(operation: &Operation, context: &mut Context) -> u16 {
    match operation {
        Operation::And(left, right) => {
            eval_term(left, context) & eval_term(right, context)
        }
        Operation::Or(left, right) => {
            eval_term(left, context) | eval_term(right, context)
        }
        Operation::Lshift(left, right) => {
            eval_term(left, context) << eval_term(right, context)
        }
        Operation::Rshift(left, right) => {
            eval_term(left, context) >> eval_term(right, context)
        }
        Operation::Not(term) => !eval_term(term, context),
    }
}

fn eval_term(term: &Term, context: &mut Context) -> u16 {
    match term {
        Term::Value(value) => value.clone(),
        Term::Variable(variable) => {
            let expression = context.get(variable).unwrap().clone();
            let result = eval_expression(&expression, context);
            context.insert(
                variable.to_string(),
                Expression::Term(Term::Value(result)),
            );
            result
        }
    }
}

fn eval_expression(expression: &Expression, context: &mut Context) -> u16 {
    match expression {
        Expression::Term(term) => eval_term(term, context),
        Expression::Operation(operation) => eval_operation(operation, context),
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let variable = "a".to_owned();
    let mut context = make_context(&input);
    let result = eval_expression(
        &Expression::Term(Term::Variable(variable)),
        &mut context,
    );
    println!("1: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expression() {
        assert_eq!(parse_expression("123"), Expression::Term(Term::Value(123)));
        assert_eq!(
            parse_expression("x AND y"),
            Expression::Operation(Operation::And(
                Term::Variable("x".to_string()),
                Term::Variable("y".to_string())
            ))
        );
        assert_eq!(
            parse_expression("NOT y"),
            Expression::Operation(Operation::Not(Term::Variable(
                "y".to_string()
            )))
        );
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("456 -> y"),
            ("y".to_string(), Expression::Term(Term::Value(456)))
        );
        assert_eq!(
            parse_line("x AND y -> d"),
            (
                "d".to_string(),
                Expression::Operation(Operation::And(
                    Term::Variable("x".to_string()),
                    Term::Variable("y".to_string())
                ))
            )
        );
        assert_eq!(
            parse_line("NOT y -> i"),
            (
                "i".to_string(),
                Expression::Operation(Operation::Not(Term::Variable(
                    "y".to_string()
                )))
            )
        );
    }

    #[test]
    fn test_make_context() {
        let test_input = "123 -> x
456 -> y
x AND y -> d
NOT d -> h";

        let expected: Context = vec![
            ("x".to_string(), Expression::Term(Term::Value(123))),
            ("y".to_string(), Expression::Term(Term::Value(456))),
            (
                "d".to_string(),
                Expression::Operation(Operation::And(
                    Term::Variable("x".to_string()),
                    Term::Variable("y".to_string()),
                )),
            ),
            (
                "h".to_string(),
                Expression::Operation(Operation::Not(Term::Variable(
                    "d".to_string(),
                ))),
            ),
        ]
        .into_iter()
        .collect();
        let actual = make_context(test_input);
        assert_eq!(actual, expected);
    }
}
