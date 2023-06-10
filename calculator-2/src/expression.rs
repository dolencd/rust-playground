use std::fmt::Display;

use anyhow::{bail, Ok};

use crate::{operation::Operation, token::Token};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f32),
    Expression(Box<Operation>),
}

pub trait Solve {
    fn solve(&self) -> f32;
}

impl Solve for Expression {
    fn solve(&self) -> f32 {
        match self {
            Expression::Number(num) => *num,
            Expression::Expression(exp) => exp.solve(),
        }
    }
}

impl TryFrom<&[Token]> for Expression {
    type Error = anyhow::Error;
    fn try_from(input: &[Token]) -> Result<Self, Self::Error> {
        let easy_output = match input {
            [] => bail!("No input"),
            [Token::Number(num)] => Some(Expression::Number(*num)),
            [Token::Number(_), Token::Asterisk | Token::Minus | Token::Plus | Token::Slash, Token::Number(_)] => {
                Some(Expression::from(Operation::try_from(input)?))
            }
            [_] | [_, _] | [_, _, _] | [_, _, _, _] => {
                bail!("Tokens are not a valid expression: {:?}", input)
            }
            _ => None,
        };

        if let Some(output) = easy_output {
            return Ok(output);
        }

        Ok(Expression::from(Operation::try_from(input)?))
    }
}

impl TryFrom<&Token> for Expression {
    type Error = anyhow::Error;
    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        if let Token::Number(num) = value {
            return Ok(Self::Number(*num));
        };
        bail!("Token is not a number {:?}", value)
    }
}

impl From<Operation> for Expression {
    fn from(exp: Operation) -> Self {
        Self::Expression(Box::new(exp))
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::Expression(exp) => write!(f, "({})", exp),
        }
    }
}
