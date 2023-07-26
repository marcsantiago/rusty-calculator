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
    #[error("Nothing to process")]
    NoInputError,
    #[error("Insufficient amount of operators")]
    OperatorError,
    #[error("Insufficient amount of arguments")]
    ArgumentsError,
}

struct Calculator {
    operators_by_precedence: HashMap<char, u8>,
}

impl Calculator {
    fn new() -> Self {
        Calculator {
            operators_by_precedence: HashMap::from([('+', 1), ('-', 1), ('*', 2), ('/', 2)]),
        }
    }

    fn parse_input(&self, input: &str) -> AnyResult<Vec<String>, AnyError> {
        let mut index = 0;
        let mut output: Vec<String> = Vec::new();
        let mut operators: Vec<String> = Vec::new();

        let tokens: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
        while index < tokens.len() {
            let current_token = tokens[index];
            if current_token.is_ascii_digit() {
                let (new_index, num_s) = Self::handle_parsing_digits(index, &tokens);
                output.push(num_s);
                index = new_index;
            } else if self.operators_by_precedence.contains_key(&current_token) {
                self.handle_parsing_operators(&current_token, &mut operators, &mut output);
                operators.push(current_token.to_string());
            } else if current_token == '(' {
                operators.push(current_token.to_string());
            } else if current_token == ')' {
                Self::handle_parsing_closing_parenthesis(&mut operators, &mut output)?;
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
        &self,
        current_token: &char,
        operators: &mut Vec<String>,
        output: &mut Vec<String>,
    ) {
        let mut operator_index = operators.len() as isize;
        if operator_index == 0 {
            return;
        }
        operator_index -= 1;
        while operators.len() > 0
            && operators[operator_index as usize] != "("
            && self.operators_by_precedence.get(current_token)
                <= self
                    .operators_by_precedence
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
        while operator_index >= 0
            && &operators[operator_index as usize].chars().next().unwrap() != &'('
        {
            output.push(operators.pop().unwrap());
            operator_index -= 1;
        }

        if operator_index < 0 {
            return Err(ProcessingError::CloseParenthesesError(operator_index))?;
        }
        operators.pop();
        Ok(())
    }

    fn process_rpn(&self, parsed_tokens: Vec<String>) -> AnyResult<f64, AnyError> {
        if parsed_tokens.len() == 0 {
            return Err(ProcessingError::NoInputError)?;
        }

        let mut index: usize = 0;
        let mut values: Vec<String> = Vec::new();
        while index < parsed_tokens.len() {
            let current_token = &parsed_tokens[index];
            match current_token.parse::<f64>() {
                Ok(_) => {
                    values.push(current_token.clone());
                }
                Err(_) => {
                    if self
                        .operators_by_precedence
                        .contains_key(&current_token.chars().next().unwrap())
                    {
                        if values.len() < 2 {
                            Err(ProcessingError::ArgumentsError)?
                        }

                        let right = Self::string_to_float(values.pop());
                        let left = Self::string_to_float(values.pop());

                        match current_token.as_str() {
                            "+" => values.push((left + right).to_string()),
                            "-" => values.push((left - right).to_string()),
                            "*" => values.push((left * right).to_string()),
                            "/" => values.push((left / right).to_string()),
                            &_ => {}
                        }
                    }
                }
            }
            index += 1;
        }

        if values.len() == 1 {
            let value = Self::string_to_float(Some(values[0].clone()));
            return Ok(value);
        }
        Err(ProcessingError::OperatorError)?
    }

    fn string_to_float(value: Option<String>) -> f64 {
        if let Some(v) = value {
            return v.parse::<f64>().unwrap_or(0.0);
        }
        0.0
    }

    pub fn evaluate(&self, input: String) -> AnyResult<f64> {
        let parsed_data = self.parse_input(&input)?;
        let result = self.process_rpn(parsed_data)?;
        Ok(result)
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        let cal = Calculator::new();

        let res = cal.parse_input("3 + 4 + 5").unwrap();
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

        // let res = cal.parse_input("3.0 + 4 + 5").unwrap();
        // assert_eq!(
        //     res,
        //     vec![
        //         String::from("3.0"),
        //         String::from("4"),
        //         String::from("+"),
        //         String::from("5"),
        //         String::from("+")
        //     ]
        // );

        let res = cal.parse_input("3+4").unwrap();
        assert_eq!(
            res,
            vec![String::from("3"), String::from("4"), String::from("+"),]
        );

        let res = cal.parse_input("3+4*5").unwrap();
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

        let res = cal.parse_input("3*4+5").unwrap();
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

        let res = cal.parse_input("(3+4)*5").unwrap();
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

        let res = cal.parse_input("(15 + 7) / 2 - (65 - 61) * 2").unwrap();
        assert_eq!(
            res,
            vec![
                String::from("15"),
                String::from("7"),
                String::from("+"),
                String::from("2"),
                String::from("/"),
                String::from("65"),
                String::from("61"),
                String::from("-"),
                String::from("2"),
                String::from("*"),
                String::from("-"),
            ]
        );

        let res = cal.parse_input("(11 + 13 * 2");
        assert!(res.is_err());
        let res = cal.parse_input("11 + 13) * 2");
        assert!(res.is_err());
    }

    #[test]
    fn test_rpn() {
        let cal = Calculator::new();
        let parsed_data = cal.parse_input("5+5").unwrap();
        let result = cal.process_rpn(parsed_data).unwrap();
        assert_eq!(result, 10.0);

        let cal = Calculator::new();
        let parsed_data = cal.parse_input("3+4*5").unwrap();
        let result = cal.process_rpn(parsed_data).unwrap();
        assert_eq!(result, 23.0);

        let cal = Calculator::new();
        let parsed_data = cal.parse_input("(15 + 7) / 2 - (65 - 61) * 2").unwrap();
        let result = cal.process_rpn(parsed_data).unwrap();
        assert_eq!(result, 3.0);
    }

    // #[test]
    // fn test_evaluate() {
    //     let cal = Calculator::new();
    //     let result = cal
    //         .evaluate(String::from("(15 + 7) / 2 - (65 - 61) * 2"))
    //         .unwrap();
    //     assert_eq!(result, 3.0);
    // }
}
