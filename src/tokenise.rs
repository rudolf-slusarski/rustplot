#![allow(dead_code)] // for now
use std::iter::Peekable;

pub struct Equation {
    pub formula: String,
    variable_count: u32,
    special_functions: Vec<SpecialFunction>,
}

impl Equation {
    pub fn new(formula: String) -> Self {
        Self {
            formula,
            variable_count: 0,
            special_functions: vec![],
        }
    }
}

enum TokenType {
    Paren,
    Multiplication,
    Addition,
    Number(u32),
    Variable(char),
    Function(SpecialFunction),
    Comma,

    // hopefully i can simplify these later on
    Subtraction,
    Division,
    Exponentiation,
}

enum SpecialFunction {
    Sine,
    Cosine,
    Tangent,
    Absolute,
}

struct Node {
    children: Vec<Node>,
    token: TokenType,
}

impl Node {
    fn new(token: TokenType) -> Self {
        Self {
            children: vec![],
            token,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Paren(char),
    Operation(char),
    Number(u32),
    Variable(char),
}

pub fn tokenise(equation: &String) -> Result<Vec<Token>, String> {
    let mut result = vec![];
    let mut iter = equation.chars().peekable();

    while let Some(&c) = iter.peek() {
        match c {
            // numbers
            '0'..='9' => {
                iter.next();
                let number = check_for_more_digits(c, &mut iter);
                result.push(Token::Number(number));
            }

            // operations
            '+' | '-' | '*' | '/' | '^' => {
                result.push(Token::Operation(c));
                iter.next();
            }

            // parentheses
            '(' | ')' | '[' | ']' | '{' | '}' => {
                result.push(Token::Paren(c));
                iter.next();
            }

            // ignore spaces
            ' ' => {
                iter.next();
            }

            // todo: variable(s)
            'x' => {
                result.push(Token::Variable('x'));
                iter.next();
            }

            // error handling
            _ => return Err(format!("unexpected character: {}", c)),
        }
    }

    Ok(result)
}

fn check_for_more_digits<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> u32 {
    let mut number = c.to_digit(10).unwrap();
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<u32>()) {
        number = number * 10 + digit;
        iter.next();
    }
    number
}

fn parse_token(tokens: &Vec<Token>, pos: usize) -> Result<(Node, usize), String> {
    let s: &Token = tokens.get(pos).unwrap();
    // this .unwrap() is wrong (also cheating)
    match *s {
        Token::Number(n) => {
            let node = Node::new(TokenType::Number(n));
            Ok((node, pos + 1))
        }

        Token::Operation(c) => {
            let node = match c {
                '+' => Node::new(TokenType::Addition),
                '*' => Node::new(TokenType::Multiplication),
                '-' => Node::new(TokenType::Subtraction),
                '/' => Node::new(TokenType::Division),
                '^' => Node::new(TokenType::Exponentiation),
                _ => todo!(),
            };
            Ok((node, pos + 1))
        }

        // parentheses ???
        Token::Paren(_) => todo!(),

        _ => todo!(),
    }
}

/// verify the order and matching of parentheses
fn verify_parentheses(tokens: &Vec<Token>) -> Result<(), String> {
    // [parens, brackets, braces]
    let mut open_parens = [0, 0, 0];

    for t in tokens {
        if let Token::Paren(c) = t {
            match c {
                '(' => open_parens[0] += 1,
                '[' => open_parens[1] += 1,
                '{' => open_parens[2] += 1,
                ')' => open_parens[0] -= 1,
                ']' => open_parens[1] -= 1,
                '}' => open_parens[2] -= 1,
                _ => (),
            };
            if open_parens.contains(&-1) {
                return Err("parentheses not matching".to_string());
            }
        }
    }
    Ok(())
}

// this should return something like an array (vector?) of coefficients sorted by degree of x
// fn evaluate_nodes(tree: Node) -> Vec<i32> {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use crate::tokenise::{tokenise, verify_parentheses, Token};

    #[test]
    fn parentheses_not_matching() {
        let parens = vec![Token::Paren(')'), Token::Paren('(')];
        assert_ne!(Ok(()), verify_parentheses(&parens));
    }

    #[test]
    fn symbol_parentheses() {
        assert_eq!(tokenise(&String::from("(")).unwrap()[0], Token::Paren('('));
    }

    #[test]
    fn symbol_space() {
        assert_eq!(
            tokenise(&String::from("    (")).unwrap()[0],
            Token::Paren('(')
        );
    }

    #[test]
    fn symbol_two_parens() {
        let eq = tokenise(&String::from("()")).unwrap();
        assert_eq!(eq[0], Token::Paren('('));
        assert_eq!(eq[1], Token::Paren(')'));
    }

    #[test]
    fn symbol_digit() {
        assert_eq!(tokenise(&String::from("1")).unwrap()[0], Token::Number(1));
    }

    #[test]
    fn symbol_number() {
        assert_eq!(
            tokenise(&String::from("100031")).unwrap()[0],
            Token::Number(100031)
        );
    }

    #[test]
    fn symbol_two_numbers() {
        let eq = tokenise(&String::from("31      13")).unwrap();
        assert_eq!(eq[0], Token::Number(31));
        assert_eq!(eq[1], Token::Number(13));
    }
}
