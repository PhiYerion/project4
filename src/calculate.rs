use crate::op::PRECEDENCE;
use crate::token::{CalcToken, ComplexToken};
use colored::*;

/// Indicates that a type can be calculated to an f64.
pub trait Calculate {
    fn calc(&mut self) -> Result<f64, String>;
}

/// Returns a string that prints the partially evaluated expression at the time of error with the
/// erronious token highlighted in red.
///
/// # Arguments
/// * `tokens` - A slice of tokens to be printed.
/// * `err_idx` - The index of the token that caused the error.
fn err_msg<T: Copy>(tokens: &[T], err_idx: usize) -> String
where
    String: From<T>,
{
    tokens.iter().enumerate().map(|(i, t)|
        if i == err_idx {
            String::from(*t).red().to_string()
        } else {
            String::from(*t)
        } + " "
    ).collect::<String>()
}

impl Calculate for Vec<ComplexToken> {
    /// Evaluates the expression taking into considering Parentheses and precedence. Does not
    /// handle the case where there is a operator at the beginning of an expression
    fn calc(&mut self) -> Result<f64, String> {
        let mut stack: Vec<ComplexToken> = Vec::new();

        for (i, token) in self.iter().enumerate() {
            match token {
                ComplexToken::RParen => {
                    let enclosure = &mut stack.split_off(
                        stack
                            .iter()
                            .rposition(|t| *t == ComplexToken::LParen)
                            .ok_or(
                                "No matching left paren:\n".to_string() + &err_msg(&stack, i),
                            )?,
                    )[1..];

                    if let Some(invalid_pos) = enclosure.iter().position(|t| !t.is_calctoken()) {
                        return Err("Token not allowed here:\n".to_string()
                            + &err_msg(enclosure, invalid_pos));
                    }

                    let mut checked_enclosure = enclosure
                        .iter_mut()
                        .map(|t| match t {
                            ComplexToken::Calc(token) => *token,
                            _ => panic!("Invalid token in enclosure"),
                        })
                        .collect::<Vec<CalcToken>>();

                    let result = checked_enclosure.calc()?;

                    stack.push(ComplexToken::Calc(CalcToken::Num(result)));
                }
                _ => stack.push(*token),
            }
        }

        if let Some(invalid_pos) = stack.iter().position(|t| !t.is_calctoken()) {
            return Err("Token not allowed here:\n".to_string() + &err_msg(&stack, invalid_pos));
        }

        let mut remaining = stack
            .iter()
            .map(|t| match t {
                ComplexToken::Calc(token) => *token,
                _ => panic!("Invalid token in stack"),
            })
            .collect::<Vec<CalcToken>>();

        remaining.calc()
    }
}

impl Calculate for Vec<CalcToken> {
    fn calc(&mut self) -> Result<f64, String> {
        if self.is_empty() {
            return Err("Empty expression".to_string());
        }

        for pre in PRECEDENCE {
            let mut i = 0;
            while i < self.len() {
                if let CalcToken::Op(op) = self[i]
                    && pre.contains(&op)
                {
                    if i == 0 || i == self.len() - 1 {
                        return Err(
                            "Operator without matching numbers:\n".to_string() + &err_msg(self, i)
                        );
                    }

                    let b = match self.remove(i + 1) {
                        CalcToken::Num(num) => num,
                        CalcToken::Op(op) => {
                            self.insert(i + 1, CalcToken::Op(op));
                            return Err("Invalid token after operator:\n".to_string()
                                + &err_msg(self, i + 1));
                        }
                    };

                    let a = match self.remove(i - 1) {
                        CalcToken::Num(num) => num,
                        CalcToken::Op(op) => {
                            self.insert(i - 1, CalcToken::Op(op));
                            return Err("Invalid token before operator:\n".to_string()
                                + &err_msg(self, i - 1));
                        }
                    };

                    let result = op.operate(a, b);

                    self[i - 1] = CalcToken::Num(result);
                } else {
                    i += 1;
                }
            }
        }

        if let CalcToken::Num(num) = self[0]
            && self.len() == 1
        {
            Ok(num)
        } else {
            Err("No coresponding operators:\n".to_string()
                + &self
                    .iter()
                    .map(|t| String::from(*t).red().to_string() + " ")
                    .collect::<String>())
        }
    }
}
