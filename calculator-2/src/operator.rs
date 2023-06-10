use std::fmt::Display;

use anyhow::bail;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl TryFrom<&Token> for Operator {
    type Error = anyhow::Error;
    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        Ok(match value {
            Token::Plus => Operator::Add,
            Token::Minus => Operator::Subtract,
            Token::Asterisk => Operator::Multiply,
            Token::Slash => Operator::Divide,
            Token::Number(_) => bail!("Token not an operator {:?}", value),
        })
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Add => "+",
                Operator::Subtract => "-",
                Operator::Multiply => "*",
                Operator::Divide => "/",
            }
        )
    }
}
