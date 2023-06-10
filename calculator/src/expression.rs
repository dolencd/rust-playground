use crate::token::Token;
use anyhow::bail;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Expression {
    tokens: Vec<Token>,
}

impl Expression {
    pub fn new(input: &str) -> anyhow::Result<Expression> {
        let tokens_res: anyhow::Result<Vec<Token>> = input
            .split_ascii_whitespace()
            .map(|possible_token_string| possible_token_string.try_into())
            .collect();

        match tokens_res {
            Err(e) => Err(e),
            Ok(tokens) => {
                let expr = Expression { tokens };
                if !expr.is_valid() {
                    bail!("Created expression is invalid")
                }
                Ok(expr)
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        self.tokens.len() >= 3
            && self.tokens.iter().tuple_windows::<(_, _, _)>().all(|tup| {
                matches!(
                    tup,
                    (
                        Token::Number(_),
                        Token::Divide | Token::Plus | Token::Minus | Token::Multiply,
                        Token::Number(_)
                    )
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn empty_input() {
        assert_eq!(Expression::new("").unwrap(), Expression { tokens: vec![] });
    }

    #[test]
    fn single_number() {
        assert_eq!(
            Expression::new("1").unwrap(),
            Expression {
                tokens: vec![Token::Number(1.0)]
            }
        );
    }

    #[test]
    fn simple_expr() {
        assert_eq!(
            Expression::new("1 + 1").unwrap(),
            Expression {
                tokens: vec![Token::Number(1.0), Token::Plus, Token::Number(1.0)]
            }
        );
    }
}
