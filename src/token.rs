//! Contains the token types used in the calculator. There are very nested enums up to
//! Token::Complex(ComplexToken::Calc(CalcToken::Op(Op::Add))), which can be a bit ugly
//! at times, but it is due to the approach that was taken.
//!
//! We treat incoming strings as a stream of tokens. These tokens can do a variety of things not
//! limited to calculations. We approach this like peeling an onion. [The outermost layer](Token) is
//! state/calculator specific operations, like assignment. The [next layer](ComplexToken) is where
//! more complex ordering than EDMAS (Parentheses) is handled. The [innermost layer](CalcToken) is
//! where EDMAS is handled.

use colored::*;
use crate::op::Op;

/// This is where EDMAS is handled
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CalcToken {
    Op(Op),
    Num(f64),
}

impl From<CalcToken> for String {
    fn from(token: CalcToken) -> String {
        match token {
            CalcToken::Op(op) => String::from(op),
            CalcToken::Num(num) => num.to_string(),
        }
    }
}

/// This is where Parentheses are handled
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComplexToken {
    Calc(CalcToken),
    LParen,
    RParen,
}

impl From<ComplexToken> for String {
    fn from(token: ComplexToken) -> String {
        match token {
            ComplexToken::Calc(token) => String::from(token),
            ComplexToken::LParen => "(".to_string(),
            ComplexToken::RParen => ")".to_string(),
        }
    }
}

impl ComplexToken {
    pub fn is_calctoken(&self) -> bool {
        matches!(self, ComplexToken::Calc(_))
    }
}

/// Where assignment is handled
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Complex(ComplexToken),
    Var(String),
    Assign,
}

impl Token {
    pub fn from_str(s: &str) -> Option<Token> {
        let into_tokenop = |op: Op| Some(Token::Complex(ComplexToken::Calc(CalcToken::Op(op))));
        match s {
            "+" => into_tokenop(Op::Add),
            "-" => into_tokenop(Op::Sub),
            "*" => into_tokenop(Op::Mul),
            "/" => into_tokenop(Op::Div),
            "%" => into_tokenop(Op::Mod),
            "^" => into_tokenop(Op::Pow),
            "=" => Some(Token::Assign),
            "(" => Some(Token::Complex(ComplexToken::LParen)),
            ")" => Some(Token::Complex(ComplexToken::RParen)),
            _ => {
                if let Ok(num) = s.parse::<f64>() {
                    Some(Token::Complex(ComplexToken::Calc(
                        CalcToken::Num(num),
                    )))
                } else {
                    Some(Token::Var(s.to_string()))
                }
            }
        }
    }
}

impl From<&Token> for String {
    fn from(token: &Token) -> String {
        match token {
            Token::Complex(ComplexToken::Calc(token)) => String::from(*token),
            Token::Complex(ComplexToken::LParen) => "(".to_string(),
            Token::Complex(ComplexToken::RParen) => ")".to_string(),
            Token::Var(var) => var.to_string(),
            Token::Assign => "=".to_string(),
        }
    }
}

pub trait TryInto<T> {
    type Error;
    fn into_calc(self) -> Result<T, Self::Error>;
}

impl TryInto<Vec<ComplexToken>> for &[Token] {
    type Error = String;

    fn into_calc(self) -> Result<Vec<ComplexToken>, Self::Error> {
        self.iter()
            .map(|t| match t {
                Token::Complex(token) => Ok(*token),
                _ => Err(self
                    .iter()
                    .map(|t| match t {
                        Token::Complex(t) => String::from(*t),
                        _ => String::from(t).red().to_string(),
                    })
                    .collect::<String>()),
            })
            .collect()
    }
}
