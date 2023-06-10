mod expression;
mod operation;
mod operator;
mod token;

use anyhow::Result;
use expression::Expression;
use token::Token;

use crate::expression::Solve;

fn main() -> Result<()> {
    let tokens = "8 * 9 - 10 / 4 * 3 + 12.3 - 4.2 / 31 * 123"
        .split_ascii_whitespace()
        .map(|possible_token| possible_token.try_into())
        .collect::<Result<Vec<Token>>>()?;

    let exp = Expression::try_from(&tokens[..])?;
    println!("Expression: {}", exp);
    println!("Solution: {}", exp.solve());
    return Ok(());
}
