use core::panic;
use std::collections::HashMap;
use std::fs;

type Context = HashMap<Term, Expression>;

#[derive(Eq, Hash, Debug, PartialEq)]
enum Term {
    Variable(String),
    Value(i32),
}

#[derive(Debug, PartialEq)]
enum Operation {
    And(Term, Term),
    Or(Term, Term),
    Lshift(Term, Term),
    Rshift(Term, Term),
    Not(Term),
}

#[derive(Debug, PartialEq)]
enum Expression {
    Term(Term),
    Operation(Operation),
}

fn parse_term(term: &str) -> Term {
    if let Ok(number) = term.parse::<i32>() {
        Term::Value(number)
    } else {
        Term::Variable(term.to_string())
    }
}

fn parse_operation(operation: &str) -> Operation {
    let parts = operation.split(" ").collect::<Vec<&str>>();

    match parts.as_slice() {
        [left, "AND", right] => Operation::And(parse_term(left), parse_term(right)),
        [left, "OR", right] => Operation::Or(parse_term(left), parse_term(right)),
        [left, "LSHIFT", right] => Operation::Lshift(parse_term(left), parse_term(right)),
        [left, "RSHIFT", right] => Operation::Rshift(parse_term(left), parse_term(right)),
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

fn parse_line(line: &str) -> (Term, Expression) {
    let parts = line.split(" -> ").collect::<Vec<&str>>();

    match parts.as_slice() {
        [expression, variable] => {
            let variable = Term::Variable(variable.to_string());
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

fn eval(expression: Expression, context: Context) {
    todo!()
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let context = make_context(&input);
    let result = eval(Expression::Term(Term::Variable("a".to_string())), context);
    println!("{:?}", result);
    // println!("{:?}", test_input);
    // let text = fs::read_to_string("./input.txt").unwrap();
    // let instructions = text.lines().map(line_to_instruction).collect::<Vec<_>>();
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
            Expression::Operation(Operation::Not(Term::Variable("y".to_string())))
        );
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("456 -> y"),
            (
                Term::Variable("y".to_string()),
                Expression::Term(Term::Value(456))
            )
        );
        assert_eq!(
            parse_line("x AND y -> d"),
            (
                Term::Variable("d".to_string()),
                Expression::Operation(Operation::And(
                    Term::Variable("x".to_string()),
                    Term::Variable("y".to_string())
                ))
            )
        );
        assert_eq!(
            parse_line("NOT y -> i"),
            (
                Term::Variable("i".to_string()),
                Expression::Operation(Operation::Not(Term::Variable("y".to_string())))
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
            (
                Term::Variable("x".to_string()),
                Expression::Term(Term::Value(123)),
            ),
            (
                Term::Variable("y".to_string()),
                Expression::Term(Term::Value(456)),
            ),
            (
                Term::Variable("d".to_string()),
                Expression::Operation(Operation::And(
                    Term::Variable("x".to_string()),
                    Term::Variable("y".to_string()),
                )),
            ),
            (
                Term::Variable("h".to_string()),
                Expression::Operation(Operation::Not(Term::Variable("d".to_string()))),
            ),
        ]
        .into_iter()
        .collect();
        let actual = make_context(test_input);
        assert_eq!(actual, expected);
    }
}
