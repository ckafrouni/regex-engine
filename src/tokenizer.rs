use super::errors;

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Char { val: Char, pos: usize },
    Quantifier { val: Quantifier, pos: usize },
    Anchor { val: Anchor, pos: usize },
}

impl Token {
    pub fn pos(&self) -> usize {
        match self {
            Token::Char { pos, .. } => *pos,
            Token::Quantifier { pos, .. } => *pos,
            Token::Anchor { pos, .. } => *pos,
        }
    }
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
    Escape(EscapeChar),
}

#[derive(Debug, Clone, Copy)]
pub enum EscapeChar {
    Digit,
    NotDigit,
    Space,
    NotSpace,
    Word,
    NotWord,
    Newline,
    Tab,
    Null,
}

impl EscapeChar {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'd' => Some(Self::Digit),
            'D' => Some(Self::NotDigit),
            's' => Some(Self::Space),
            'S' => Some(Self::NotSpace),
            'w' => Some(Self::Word),
            'W' => Some(Self::NotWord),
            'n' => Some(Self::Newline),
            't' => Some(Self::Tab),
            '0' => Some(Self::Null),
            _ => None,
        }
    }
}

pub fn tokenize(pattern: String) -> Result<Vec<Token>, errors::ParseError> {
    let mut tokens: Vec<Token> = vec![];

    let mut chars = pattern.char_indices();
    while let Some((pos, val)) = chars.next() {
        let token = match val {
            '\\' => match chars.next() {
                Some((_, '\\')) => Token::Char {
                    val: Char::Lit('\\'),
                    pos,
                },
                Some((_, '$')) => Token::Char {
                    val: Char::Lit('$'),
                    pos,
                },
                Some((_, '^')) => Token::Char {
                    val: Char::Lit('^'),
                    pos,
                },
                Some((_, '.')) => Token::Char {
                    val: Char::Lit('.'),
                    pos,
                },
                Some((_, '*')) => Token::Char {
                    val: Char::Lit('*'),
                    pos,
                },
                Some((_, '+')) => Token::Char {
                    val: Char::Lit('+'),
                    pos,
                },
                Some((_, '?')) => Token::Char {
                    val: Char::Lit('?'),
                    pos,
                },
                Some((_, c)) => {
                    if let Some(escape_char) = EscapeChar::from_char(c) {
                        Token::Char {
                            val: Char::Escape(escape_char),
                            pos,
                        }
                    } else {
                        return Err(errors::ParseError::BadEscapeSequence(
                            pos,
                            format!("Unknown escape sequence: \\{}", c),
                        ));
                    }
                }
                None => {
                    return Err(errors::ParseError::BadEscapeSequence(
                        pos,
                        "Expected a character after the escape sequence".into(),
                    ))
                }
            },
            '.' => Token::Char {
                val: Char::Dot,
                pos,
            },
            '*' => Token::Quantifier {
                val: Quantifier::Any,
                pos,
            },
            '+' => Token::Quantifier {
                val: Quantifier::Many,
                pos,
            },
            '?' => Token::Quantifier {
                val: Quantifier::Maybe,
                pos,
            },
            '^' => Token::Anchor {
                val: Anchor::Start,
                pos,
            },
            '$' => Token::Anchor {
                val: Anchor::End,
                pos,
            },
            c => Token::Char {
                val: Char::Lit(c),
                pos,
            },
        };

        tokens.push(token);
    }

    Ok(tokens)
}
