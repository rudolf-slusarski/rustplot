use crate::SpecialFunction;

enum Token {
    Paren,
    Product,
    Sum,
    Number(i64),
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

pub enum Symbol {
    Paren(char),
    Operation(char),
    Number(u8),
}

pub fn split_into_symbols(equation: &String) -> Vec<Symbol> {
    let mut result = vec![];
    let mut it = equation.chars().peekable();

    while let Some(&c) = it.peek() {
        match c {
            // operations
            '+' | '-' | '*' | '/' | '^'  => {
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

            // numbers and error handling
            _ => todo!(),
        }
    }

    result
}
