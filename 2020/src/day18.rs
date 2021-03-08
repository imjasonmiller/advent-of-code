use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unrecognized token: {0}")]
    UnrecognizedToken(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Num(i64),
    Mul,
    Add,
    ParenOpen,
    ParenClose,
}

#[derive(Default, Clone, Copy)]
pub struct Tokens<'a> {
    source: &'a [u8],
    start: usize,
    end: usize,
}

impl<'a> Tokens<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            source: s.as_bytes(),
            ..Default::default()
        }
    }

    fn peek(&mut self) -> Option<Token> {
        let prev_start = self.start;
        let prev_end = self.end;

        let result = self.next();

        self.start = prev_start;
        self.end = prev_end;

        result
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // Return if cursor is at the end of source
        if self.end == self.source.len() {
            return None;
        }

        // Consume leading whitespace
        while self.source[self.end].is_ascii_whitespace() {
            self.end += 1;
        }

        // Align start and end cursor
        self.start = self.end;

        // Match token
        self.end += 1;
        match self.source[self.start] {
            b'+' => Some(Token::Add),
            b'*' => Some(Token::Mul),
            b'(' => Some(Token::ParenOpen),
            b')' => Some(Token::ParenClose),
            b'0'..=b'9' => {
                // Move cursor to the last digit
                while self.end < self.source.len() && self.source[self.end].is_ascii_digit() {
                    self.end += 1;
                }

                let n = str::from_utf8(&self.source[self.start..self.end])
                    .unwrap()
                    .parse::<i64>()
                    .unwrap();

                Some(Token::Num(n))
            }
            t => panic!("Unexpected token: \"{}\"", t),
        }
    }
}

use std::str;

pub trait Evaluable: std::fmt::Debug {
    fn eval(&self) -> i64;
}

#[derive(Debug)]
enum BinaryExpr {
    Add(Box<dyn Evaluable>, Box<dyn Evaluable>),
    Mul(Box<dyn Evaluable>, Box<dyn Evaluable>),
}

impl BinaryExpr {
    fn add(lhs: Box<dyn Evaluable>, rhs: Box<dyn Evaluable>) -> Box<Self> {
        Box::new(Self::Add(lhs, rhs))
    }

    fn mul(lhs: Box<dyn Evaluable>, rhs: Box<dyn Evaluable>) -> Box<Self> {
        Box::new(Self::Mul(lhs, rhs))
    }
}

impl Evaluable for BinaryExpr {
    fn eval(&self) -> i64 {
        match self {
            BinaryExpr::Add(lhs, rhs) => lhs.eval() + rhs.eval(),
            BinaryExpr::Mul(lhs, rhs) => lhs.eval() * rhs.eval(),
        }
    }
}

#[derive(Debug)]
struct Literal(i64);

impl Literal {
    fn new(value: i64) -> Box<Self> {
        Box::new(Self(value))
    }
}

impl Evaluable for Literal {
    fn eval(&self) -> i64 {
        self.0
    }
}

pub type ParseResult = Result<Box<dyn Evaluable>, ParseError>;

pub mod parser1 {
    use super::Token::*;
    use super::*;

    /// For the first part of this exercise, the order of operations does not matter
    ///
    /// Using the following grammar:
    ///
    /// E  : T, E'
    ///    ;
    /// E' : "+", T, E'
    ///    | '*', T, E'
    ///    | ε
    ///    ;
    /// T  : N
    ///    | "(" E ")"
    ///    ;
    /// N  : number
    ///    ;
    ///
    /// We can parse expressions such as:
    ///
    /// ```
    /// assert_eq!(parse_expr("1"), 1);
    /// assert_eq!(parse_expr("2 + 3"), 5);
    /// assert_eq!(parse_expr("(5 + 3) + 4"), 12);
    /// assert_eq!(parse_expr("4 * (3 + 2)"), 20);
    /// ```
    ///
    /// Note how operator precedence does not matter:
    /// ````
    /// assert_eq!(parse_expr("2 + 3 * 4"), 20);
    /// ```

    pub fn parse_expr(tokens: &mut Tokens) -> ParseResult {
        let expr = parse_term(tokens)?;

        match tokens.peek() {
            Some(Add) | Some(Mul) => parse_expr1(expr, tokens),
            _ => Ok(expr),
        }
    }

    fn parse_expr1(lhs: Box<dyn Evaluable>, tokens: &mut Tokens) -> ParseResult {
        match tokens.next() {
            Some(Add) => {
                let expr = BinaryExpr::add(lhs, parse_term(tokens)?);

                match tokens.peek() {
                    Some(Add) | Some(Mul) => parse_expr1(expr, tokens),
                    _ => Ok(expr),
                }
            }
            Some(Mul) => {
                let expr = BinaryExpr::mul(lhs, parse_term(tokens)?);

                match tokens.peek() {
                    Some(Add) | Some(Mul) => parse_expr1(expr, tokens),
                    _ => Ok(expr),
                }
            }
            _ => Ok(lhs),
        }
    }

    fn parse_term(tokens: &mut Tokens) -> ParseResult {
        match tokens.next() {
            Some(Num(n)) => {
                let expr = Literal::new(n);
                Ok(expr)
            }
            Some(ParenOpen) => {
                let expr = parse_expr(tokens)?;
                tokens.next();
                Ok(expr)
            }
            t => Err(ParseError::UnrecognizedToken(format!("{:?}", t))),
        }
    }
}

pub mod parser2 {
    use super::Token::*;
    use super::*;

    /// Using the following grammar:
    ///
    /// E  : T, E'
    ///    ;
    /// E' : "*", T, E'
    ///    | ε
    ///    ;
    /// T  : F, T'
    ///    ;
    /// T' : "+", F, T'
    ///    | ε
    ///    ;
    /// F  : N
    ///    | "(" E ")"
    ///    ;
    /// N  : number
    ///    ;
    ///
    /// We can parse expressions such as:
    ///
    /// ```
    /// assert_eq!("1 + (2 * 3) + (4 * (5 + 6))", 51);
    /// assert_eq!("2 * 3 + (4 * 5)", 46);
    /// assert_eq!("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445);
    /// assert_eq!("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060);
    /// assert_eq!("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340);
    /// ```
    ///
    /// Note how addition his higher precedence than multiplication
    /// ```
    /// assert_eq!(parse_expr("3 * 4 + 2"), 18);
    /// ```

    pub fn parse_expr(tokens: &mut Tokens) -> ParseResult {
        let expr = parse_term(tokens)?;

        match tokens.peek() {
            Some(Mul) => parse_expr1(expr, tokens),
            _ => Ok(expr),
        }
    }

    fn parse_expr1(lhs: Box<dyn Evaluable>, tokens: &mut Tokens) -> ParseResult {
        match tokens.next() {
            Some(Mul) => {
                let expr = BinaryExpr::mul(lhs, parse_term(tokens)?);

                match tokens.peek() {
                    Some(Mul) => parse_expr1(expr, tokens),
                    _ => Ok(expr),
                }
            }
            _ => unreachable!(),
        }
    }

    fn parse_term(tokens: &mut Tokens) -> ParseResult {
        let expr = parse_factor(tokens)?;

        match tokens.peek() {
            Some(Add) => parse_term1(expr, tokens), // BinaryExpr::add(expr, parse_term(tokens)?),
            _ => Ok(expr),
        }
    }

    fn parse_term1(lhs: Box<dyn Evaluable>, tokens: &mut Tokens) -> ParseResult {
        match tokens.next() {
            Some(Add) => {
                let expr = BinaryExpr::add(lhs, parse_factor(tokens)?);

                match tokens.peek() {
                    Some(Add) => parse_term1(expr, tokens),
                    _ => Ok(expr),
                }
            }
            _ => unreachable!(),
        }
    }

    fn parse_factor(tokens: &mut Tokens) -> ParseResult {
        match tokens.next() {
            Some(Num(n)) => Ok(Literal::new(n)),
            Some(ParenOpen) => {
                let expr = parse_expr(tokens)?;
                tokens.next(); // Consume ParenClose
                Ok(expr)
            }
            t => Err(ParseError::UnrecognizedToken(format!("{:?}", t))),
        }
    }
}

pub fn part1(input: &str) -> anyhow::Result<i64> {
    use parser1::parse_expr;

    Ok(input
        .lines()
        .map(|expr| parse_expr(&mut Tokens::new(expr)).unwrap().eval())
        .sum())
}

pub fn part2(input: &str) -> anyhow::Result<i64> {
    use parser2::parse_expr;

    Ok(input
        .lines()
        .map(|expr| parse_expr(&mut Tokens::new(expr)).unwrap().eval())
        .sum())
}
