use crate::token::{CalcToken, ComplexToken, Token};

/// Tokenizes either an expression (`(1 + 2)*3/-5`) or an assignment-expression (`x = 5 * 10` or
/// `x = 5`)
pub fn tokenize(str: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let remaining =
        str.chars().fold(String::new(), |mut acc, c| {
            match c {
                // An operator will add both the accumulator (if not empty) and operator as tokens.
                '+' | '*' | '/' | '%' | '^' | '=' | '(' | ')' => {
                    if !acc.is_empty() {
                        tokens.push(Token::from_str(&acc).unwrap());
                        acc.clear();
                    }
                    tokens.push(Token::from_str(&c.to_string()).unwrap());
                },
                // If there is a '-', but there is not a number before it, then it should mean the
                // number is negative, not a subtraction.
                '-' => {
                    if !acc.is_empty() {
                        tokens.push(Token::from_str(&acc).unwrap());
                        acc.clear();
                    }
                    match tokens.last() {
                        Some(Token::Complex(ComplexToken::Calc(CalcToken::Num(_)))) => {
                            tokens.push(Token::from_str(&c.to_string()).unwrap());
                        },
                        _ => {
                            acc.push(c);
                        }
                    }
                },
                // On whitespace, fush the accumulator to a token if it is not empty.
                ' ' | '\n' => {
                    if !acc.is_empty() {
                        tokens.push(Token::from_str(&acc).unwrap());
                        acc.clear();
                    }
                }
                _ => acc.push(c),
            }
            acc
        });
    // There may be left over characters in the accumulator that need to be pushed.
    tokens.push(Token::from_str(&remaining).unwrap());

    tokens
}
