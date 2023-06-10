use anyhow::bail;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Number(f32),
    // Expression(Expression),
}

impl TryFrom<&str> for Token {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value.trim() {
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Asterisk,
            "/" => Token::Slash,
            other => {
                let parsed_num_res = other.parse::<f32>();
                if let Ok(parsed_num) = parsed_num_res {
                    return Ok(Token::Number(parsed_num));
                }
                bail!("Couldn't parse string into token {}", other);
            }
        })
    }
}
