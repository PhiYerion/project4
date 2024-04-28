use std::collections::HashMap;

use crate::calculate::Calculate;
use crate::parse::tokenize;
use crate::token::{CalcToken, ComplexToken, Token, TryInto};
use colored::*;

#[derive(Debug, Default)]
/// Stores state of the calculator
pub struct Calculator {
    variables: HashMap<String, f64>,
}

impl Calculator {
    /// Runs the calculator in a loop
    ///
    /// Asks user for expression or assignment and stores previous state as variables.
    pub fn run(&mut self) {
        loop {
            println!();
            println!("Last result: {:?}", self.last_result());
            println!("Variables: {:?}", self.variables);
            println!("Enter an expression or assignment, or press help for more info:");
            match input() {
                Command::Calculate(tokens) => {
                    if let Err(e) = self.pass(tokens) {
                        println!("Error: {}", e);
                    }
                }
                Command::Exit => break,
                Command::Help => {
                    println!("Enter an expression to calculate it.");
                    println!("Enter an assignment to assign a value to a variable.");
                    println!("Enter 'exit' to exit the program.");
                }
            }
        }
    }

    fn last_result(&self) -> Option<f64> {
        self.variables.get("ans").copied()
    }

    /// Set a variable that the user can reference in future calculations. See [Self::extract_vars]
    /// for how this is done.
    fn set_var(&mut self, name: String, value: f64) {
        self.variables.insert(name, value);
    }

    /// Parses user input and stores result in [Calculator]'s [HashMap] or returns a [String] error
    /// to be printed to the user. If there is no assignment, the result is stored in the "ans"
    /// variable.
    fn pass(&mut self, mut tokens: Vec<Token>) -> Result<(), String> {
        // If there is an assignment token, split it into an assignment and expression part.
        // Evaluate the expression and store it in the variable indicated by the assignment token.
        // Otherwise, evaluate all the tokens and store it in the "ans" variable.
        if let Some(pos) = tokens.iter().position(|t| *t == Token::Assign) {
            let (assignment, expression) = tokens.split_at_mut(pos);

            let mut parsed_expression = self.extract_vars(&mut expression[1..])?.into_calc()?;

            let result = parsed_expression.calc()?;

            if let [Token::Var(name)] = assignment {
                self.set_var(name.to_string(), result);
            } else {
                let err_msg = assignment.iter().map(|t|
                    match t {
                        Token::Var(name) => name.to_string(),
                        _ => String::from(t).red().to_string(),
                    } + " "
                ).collect::<String>();

                return Err(err_msg);
            }
        } else {
            let mut parsed_expression = self.extract_vars(&mut tokens)?.into_calc()?;

            let result = parsed_expression.calc()?;

            self.set_var("ans".to_string(), result);
        }

        Ok(())
    }

    /// Modifies [[Token]] in place by replacing any [Token::Var] with a [CalcToken::Num] containing
    /// the value stored in [Calculator]'s [HashMap]. If a variable is not found, an error is
    /// returned.
    fn extract_vars<'a>(&self, tokens: &'a mut [Token]) -> Result<&'a mut [Token], String> {
        for t in &mut *tokens {
            if let Token::Var(name) = t {
                let var = self
                    .variables
                    .get(name)
                    .ok_or(format!("Variable {} not found", name))?;
                *t = Token::Complex(ComplexToken::Calc(CalcToken::Num(*var)));
            }
        }

        Ok(tokens)
    }
}

/// All possible commands that the user can input
enum Command {
    Calculate(Vec<Token>),
    Exit,
    Help,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "exit" => Command::Exit,
            "help" => Command::Help,
            _ => Command::Calculate(tokenize(s.to_string())),
        }
    }
}

fn input() -> Command {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    Command::from(input.trim())
}
