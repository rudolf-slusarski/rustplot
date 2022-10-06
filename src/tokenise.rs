#![allow(dead_code)] // for now
use std::{any::Any, iter::Peekable};

use crate::SpecialFunction;

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

#[derive(Debug, PartialEq)]
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
    match s {
        &Token::Number(n) => {
            let node = Node::new(TokenType::Number(n));
            Ok((node, pos + 1))
        }

        &Token::Operation(c) => {
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
        &Token::Paren(c) => match c {
            _ => todo!(),
        },

        _ => todo!(),
    }
}

fn match_parentheses(tokens: &Vec<Token>) -> bool {
    let mut open_parens = 0;
    let mut open_brackets = 0;
    let mut open_braces = 0;

    for t in tokens {
        match t {
            Token::Paren(c) => match c {
                '(' => open_parens += 1,
                '[' => open_brackets += 1,
                '{' => open_braces += 1,
                ')' => {
                    if open_parens > 0 {
                        open_parens -= 1
                    } else {
                        return false;
                    }
                }
                ']' => {
                    if open_brackets > 0 {
                        open_brackets -= 1
                    } else {
                        return false;
                    }
                }
                '}' => {
                    if open_braces > 0 {
                        open_braces -= 1
                    } else {
                        return false;
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::tokenise::{tokenise, Token};

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
