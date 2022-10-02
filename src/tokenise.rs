use std::iter::Peekable;

use crate::SpecialFunction;

enum Token {
    Paren,
    Product,
    Sum,
    Number(i64),
    Variable(char),
    Function(SpecialFunction),
}

struct Node {
    children: Vec<Node>,
    token: Token,
}

impl Node {
    fn new() -> Self {
        Self {
            children: vec![],
            token: Token::Paren,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Symbol {
    Paren(char),
    Operation(char),
    Number(u32),
}

pub fn check_for_digits<T: Iterator>(it: &Peekable<T>) -> u32 { 0 }

pub fn split_into_symbols(equation: &String) -> Vec<Symbol> {
    let mut result = vec![];
    let mut it = equation.chars().peekable();

    while let Some(&c) = it.peek() {
        match c {
            // numbers
            '0'..='9' => {
                let number = c.to_digit(10).unwrap();

                // check for more digits

                result.push(Symbol::Number(number));
                it.next();
                todo!()
            }

            // operations
            '+' | '-' | '*' | '/' | '^' => {
                result.push(Symbol::Operation(c));
                it.next();
            }

            // parentheses
            '(' | ')' | '[' | ']' | '{' | '}' => {
                result.push(Symbol::Paren(c));
                it.next();
            }

            // ignore spaces
            ' ' => {
                it.next();
            }

            // variables
            'a'..='z' => {
                todo!()
            }

            // error handling
            _ => todo!(),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::tokenise::{split_into_symbols, Symbol};

    #[test]
    fn symbol_parentheses() {
        assert_eq!(
            split_into_symbols(&String::from("("))[0],
            Symbol::Paren('(')
        );
    }

    #[test]
    fn symbol_space() {
        assert_eq!(
            split_into_symbols(&String::from("    ("))[0],
            Symbol::Paren('(')
        );
    }

    #[test]
    fn symbol_two_parens() {
        let eq = split_into_symbols(&String::from("()"));
        assert_eq!(eq[0], Symbol::Paren('('));
        assert_eq!(eq[1], Symbol::Paren(')'));
    }
}
