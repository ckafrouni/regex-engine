use crate::tokenizer::Token;

pub enum ParseError {
    UnexpectedToken(Token, String),
    BadEscapeSequence(usize, String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let msg = match self {
            ParseError::UnexpectedToken(tok, expected) => {
                format!("Unexpected token {:?}, expected {}", tok, expected)
            }
            ParseError::BadEscapeSequence(pos, expected) => {
                format!(
                    "Bad escape sequence at position {}, expected {}",
                    pos, expected
                )
            }
        };

        write!(f, "{}", msg)
    }
}

impl std::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let msg = match self {
            ParseError::UnexpectedToken(tok, expected) => {
                format!("Unexpected token {:?}, expected {}", tok, expected)
            }
            ParseError::BadEscapeSequence(pos, expected) => {
                format!(
                    "Bad escape sequence at position {}, expected {}",
                    pos, expected
                )
            }
        };

        write!(f, "{}", msg)
    }
}
