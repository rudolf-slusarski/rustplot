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
    Variable(char),
}

pub fn check_for_more_digits<T: Iterator<Item = char>>(
    mut number: u32,
    iter: &mut Peekable<T>,
) -> u32 {
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<u32>()) {
        number = number * 10 + digit;
        iter.next();
    }
    number
}

pub fn split_into_symbols(equation: &String) -> Result<Vec<Symbol>, String> {
    let mut result = vec![];
    let mut iter = equation.chars().peekable();

    while let Some(&c) = iter.peek() {
        match c {
            // numbers
            '0'..='9' => {
                iter.next();
                let number = c.to_digit(10).unwrap();
                let number = check_for_more_digits(number, &mut iter);
                result.push(Symbol::Number(number));
            }

            // operations
            '+' | '-' | '*' | '/' | '^' => {
                result.push(Symbol::Operation(c));
                iter.next();
            }

            // parentheses
            '(' | ')' | '[' | ']' | '{' | '}' => {
                result.push(Symbol::Paren(c));
                iter.next();
            }

            // ignore spaces
            ' ' => {
                iter.next();
            }
            
            // todo: variables

            // error handling
            _ => return Err(format!("unexpected character: {}", c)),

        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::tokenise::{split_into_symbols, Symbol};

    #[test]
    fn symbol_parentheses() {
        assert_eq!(
            split_into_symbols(&String::from("(")).unwrap()[0],
            Symbol::Paren('(')
        );
    }

    #[test]
    fn symbol_space() {
        assert_eq!(
            split_into_symbols(&String::from("    (")).unwrap()[0],
            Symbol::Paren('(')
        );
    }

    #[test]
    fn symbol_two_parens() {
        let eq = split_into_symbols(&String::from("()")).unwrap();
        assert_eq!(eq[0], Symbol::Paren('('));
        assert_eq!(eq[1], Symbol::Paren(')'));
    }

    #[test]
    fn symbol_digit() {
        assert_eq!(
            split_into_symbols(&String::from("1")).unwrap()[0],
            Symbol::Number(1)
        );
    }

    #[test]
    fn symbol_number() {
        assert_eq!(
            split_into_symbols(&String::from("100031")).unwrap()[0],
            Symbol::Number(100031)
        );
    }

    #[test]
    fn symbol_two_numbers() {
        let eq = split_into_symbols(&String::from("31      13")).unwrap();
        assert_eq!(eq[0], Symbol::Number(31));
        assert_eq!(eq[1], Symbol::Number(13));
    }
}
