use anyhow::bail;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    Number(f32),
    // Expression(Expression),
}

impl TryFrom<&str> for Token {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value.trim() {
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Multiply,
            "/" => Token::Divide,
            other => {
                let parsed_num_res = other.parse::<f32>();
                if let Ok(parsed_num) = parsed_num_res {
                    return Ok(Token::Number(parsed_num));
                }
                bail!("Couldn't parse string {}", other);
            }
        })
    }
}

impl Token {
    fn is_operator(token: &Token) -> bool {
        matches!(
            *token,
            Token::Divide | Token::Plus | Token::Minus | Token::Multiply
        )
    }

    fn is_value(token: &Token) -> bool {
        !Token::is_operator(token)
    }
}
