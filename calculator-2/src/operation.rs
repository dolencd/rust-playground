use std::fmt::{write, Display};

use crate::{
    expression::{Expression, Solve},
    operator::Operator,
    token::Token,
};
use anyhow::{bail, Result};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    left: Expression,
    right: Expression,
    operator: Operator,
}

impl TryFrom<&[Token]> for Operation {
    type Error = anyhow::Error;
    fn try_from(input: &[Token]) -> std::result::Result<Self, Self::Error> {
        let operators: Vec<(usize, Operator)> = input
            .iter()
            .enumerate()
            .filter_map(|(index, token)| {
                if let Ok(operator) = Operator::try_from(token) {
                    Some((index, operator))
                } else {
                    None
                }
            })
            .collect();

        let possible_low_priority_operator = operators
            .iter()
            .rev()
            .find(|(_, operator)| matches!(operator, Operator::Add | Operator::Subtract));

        let Some((index, operator)) = possible_low_priority_operator
            .map(|o| Some(o))
            .unwrap_or(operators.get(operators.len()-1)) else {
                bail!("No operators found among tokens: {:?}", input);
            };

        println!("chosen index: {} with tokens: {:?}", index, input);

        return Ok(Operation {
            left: Expression::try_from(&input[0..*index])?,
            operator: operator.to_owned(),
            right: Expression::try_from(&input[(index + 1)..input.len()])?,
        });
    }
}

impl Solve for Operation {
    fn solve(&self) -> f32 {
        let left = self.left.solve();
        let right = self.right.solve();
        match self.operator {
            Operator::Add => left + right,
            Operator::Subtract => left - right,
            Operator::Multiply => left * right,
            Operator::Divide => left / right,
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}
