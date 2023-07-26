use anyhow::{Error as AnyError, Result as AnyResult};
use std::collections::HashMap;
use thiserror::Error;
// https://stackoverflow.com/questions/40848918/are-there-queue-and-stack-collections-in-rust
// a stack may be implemented either on top of Vec or LinkedList (both feature pop_back and push_back)
// a queue may be implemented either on top of VecDeque or LinkedList (both feature pop_front and push_back)
//  In general, I would advise to use Vec for a stack and VecDeque for a queue

#[derive(Error, Debug)]
enum ProcessingError {
    #[error("Unknown token \"{0}\" at index {1}")]
    SyntaxError(char, usize),
    #[error("Could not find pair for \"(\"")]
    OpenParenthesesError,
    #[error("Could not find pair for \")\" at index {0}")]
    CloseParenthesesError(isize),
}

fn parse_input(input: &str) -> AnyResult<Vec<String>, AnyError> {
    let mut index = 0;
    let mut output: Vec<String> = Vec::new();
    let mut operators: Vec<String> = Vec::new();

    let operators_by_precedence: HashMap<char, u8> =
        HashMap::from([('+', 1), ('-', 1), ('*', 2), ('/', 2)]);

    let tokens: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
    while index < tokens.len() {
        let current_token = tokens[index];
        if current_token.is_ascii_digit() {
            let (new_index, num_s) = handle_parsing_digits(index, &tokens);
            output.push(num_s);
            index = new_index;
        } else if operators_by_precedence.contains_key(&current_token) {
            handle_parsing_operators(
                &current_token,
                &mut operators,
                &mut output,
                &operators_by_precedence,
            );
            operators.push(current_token.to_string());
        } else if current_token == '(' {
            operators.push(current_token.to_string());
        } else if current_token == ')' {
            handle_parsing_closing_parenthesis(&mut operators, &mut output)?;
            operators.pop();
        } else {
            return Err(ProcessingError::SyntaxError(current_token, index))?;
        }
        index += 1;
    }

    while !operators.is_empty() {
        let op = operators.pop().unwrap();
        if op == "(" {
            return Err(ProcessingError::OpenParenthesesError)?;
        }
        output.push(op);
    }
    Ok(output)
}

fn handle_parsing_digits(current_index: usize, tokens: &Vec<char>) -> (usize, String) {
    let mut num_s = String::new();
    let mut current_index = current_index;
    let mut token = tokens[current_index];
    num_s += &token.to_string();
    loop {
        current_index += 1;
        if current_index >= tokens.len() {
            break;
        }
        token = tokens[current_index];
        if !token.is_ascii_digit() {
            break;
        }
        num_s += &token.to_string();
    }
    (current_index - 1, num_s)
}

fn handle_parsing_operators(
    current_token: &char,
    operators: &mut Vec<String>,
    output: &mut Vec<String>,
    operators_by_precedence: &HashMap<char, u8>,
) {
    let mut operator_index = operators.len() as isize;
    if operator_index == 0 {
        return;
    }
    operator_index -= 1;
    while operators.len() > 0
        && operators[operator_index as usize] != "("
        && operators_by_precedence.get(current_token)
            <= operators_by_precedence
                .get(&operators[operator_index as usize].chars().next().unwrap())
    {
        if let Some(op) = operators.pop() {
            output.push(op);
        }
        if operator_index > 0 {
            operator_index -= 1;
        }
    }
}

fn handle_parsing_closing_parenthesis(
    operators: &mut Vec<String>,
    output: &mut Vec<String>,
) -> Result<(), AnyError> {
    let mut operator_index = operators.len() as isize;
    if operator_index == 0 {
        return Ok(());
    }

    operator_index -= 1;
    let current_op = &operators[operator_index as usize].chars().next().unwrap();
    while operator_index > 0 && current_op != &'(' {
        output.push(operators.pop().unwrap());
        operator_index -= 1;
    }

    if operator_index < 0 {
        return Err(ProcessingError::CloseParenthesesError(operator_index))?;
    }
    operators.pop();
    Ok(())
}

mod tests {
    use super::*;
    use anyhow::__private::kind::TraitKind;
    #[test]
    fn it_works() {
        let res = parse_input("3 + 4 + 5").unwrap();
        assert_eq!(
            res,
            vec![
                String::from("3"),
                String::from("4"),
                String::from("+"),
                String::from("5"),
                String::from("+")
            ]
        );

        let res = parse_input("3+4").unwrap();
        assert_eq!(
            res,
            vec![String::from("3"), String::from("4"), String::from("+"),]
        );

        let res = parse_input("3+4*5").unwrap();
        assert_eq!(
            res,
            vec![
                String::from("3"),
                String::from("4"),
                String::from("5"),
                String::from("*"),
                String::from("+")
            ]
        );

        let res = parse_input("3*4+5").unwrap();
        assert_eq!(
            res,
            vec![
                String::from("3"),
                String::from("4"),
                String::from("*"),
                String::from("5"),
                String::from("+")
            ]
        );

        let res = parse_input("(3+4)*5").unwrap();
        assert_eq!(
            res,
            vec![
                String::from("3"),
                String::from("4"),
                String::from("+"),
                String::from("5"),
                String::from("*")
            ]
        );

        let res = parse_input("(11 + 13 * 2").map_err(|e| e.anyhow_kind());
        assert!(res.is_err());
    }
}
