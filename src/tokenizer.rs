use super::errors;

#[derive(Debug)]
pub enum Token {
    Char(Char),
    Quantifier(Quantifier),
    Anchor(Anchor),
}

#[derive(Debug, Clone, Copy)]
pub enum Quantifier {
    Any,
    Many,
    Maybe,
}

#[derive(Debug, Clone, Copy)]
pub enum Anchor {
    Start,
    End,
}

#[derive(Debug, Clone, Copy)]
pub enum Char {
    Lit(char),
    Dot,
}

pub fn tokenize(pattern: String) -> Result<Vec<Token>, errors::ParseError> {
    let mut tokens: Vec<Token> = vec![];

    let mut chars = pattern.chars();
    while let Some(c) = chars.next() {
        tokens.push(match c {
            '\\' => match chars.next() {
                Some(c) => Token::Char(Char::Lit(c)),
                None => {
                    return Err(errors::ParseError::UnexpectedToken(
                        "Escape isn't escaping anything".into(),
                    ))
                }
            },
            '.' => Token::Char(Char::Dot),
            '*' => Token::Quantifier(Quantifier::Any),
            '+' => Token::Quantifier(Quantifier::Many),
            '?' => Token::Quantifier(Quantifier::Maybe),
            '^' => Token::Anchor(Anchor::Start),
            '$' => Token::Anchor(Anchor::End),
            c => Token::Char(Char::Lit(c)),
        })
    }

    Ok(tokens)
}
