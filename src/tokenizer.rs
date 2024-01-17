use super::errors;

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Quantifier {
    Any,
    Many,
    Maybe,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Anchor {
    Start,
    End,
    CharClassStart,
    CharClassEnd,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Char {
    Lit(char),
    Dot,
    Escape(EscapeChar),
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
                Some((_, '[')) => Token::Char {
                    val: Char::Lit('['),
                    pos,
                },
                Some((_, ']')) => Token::Char {
                    val: Char::Lit(']'),
                    pos,
                },
                Some((_, '(')) => Token::Char {
                    val: Char::Lit('('),
                    pos,
                },
                Some((_, ')')) => Token::Char {
                    val: Char::Lit(')'),
                    pos,
                },
                Some((_, '{')) => Token::Char {
                    val: Char::Lit('{'),
                    pos,
                },
                Some((_, '}')) => Token::Char {
                    val: Char::Lit('}'),
                    pos,
                },
                Some((_, '|')) => Token::Char {
                    val: Char::Lit('|'),
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
            '[' => Token::Anchor {
                val: Anchor::CharClassStart,
                pos,
            },
            ']' => Token::Anchor {
                val: Anchor::CharClassEnd,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let tokens = tokenize("".to_string()).unwrap();
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_normal_char() {
        assert_eq!(
            tokenize("a".to_string()).unwrap(),
            vec![Token::Char {
                val: Char::Lit('a'),
                pos: 0,
            }]
        );
    }

    // tests a bad escape sequence
    #[test]
    fn test_bad_escape() {
        tokenize("\\a".to_string()).unwrap_err();
    }

    #[test]
    fn test_correctly_escaped() {
        let test_cases = vec![
            (
                "\\[",
                Token::Char {
                    val: Char::Lit('['),
                    pos: 0,
                },
            ),
            (
                "\\]",
                Token::Char {
                    val: Char::Lit(']'),
                    pos: 0,
                },
            ),
            (
                "\\(",
                Token::Char {
                    val: Char::Lit('('),
                    pos: 0,
                },
            ),
            (
                "\\)",
                Token::Char {
                    val: Char::Lit(')'),
                    pos: 0,
                },
            ),
            (
                "\\{",
                Token::Char {
                    val: Char::Lit('{'),
                    pos: 0,
                },
            ),
            (
                "\\}",
                Token::Char {
                    val: Char::Lit('}'),
                    pos: 0,
                },
            ),
            (
                "\\|",
                Token::Char {
                    val: Char::Lit('|'),
                    pos: 0,
                },
            ),
            (
                "\\*",
                Token::Char {
                    val: Char::Lit('*'),
                    pos: 0,
                },
            ),
            (
                "\\+",
                Token::Char {
                    val: Char::Lit('+'),
                    pos: 0,
                },
            ),
            (
                "\\?",
                Token::Char {
                    val: Char::Lit('?'),
                    pos: 0,
                },
            ),
            (
                "\\^",
                Token::Char {
                    val: Char::Lit('^'),
                    pos: 0,
                },
            ),
            (
                "\\$",
                Token::Char {
                    val: Char::Lit('$'),
                    pos: 0,
                },
            ),
            (
                "\\.",
                Token::Char {
                    val: Char::Lit('.'),
                    pos: 0,
                },
            ),
        ];

        for (input, expected_token) in test_cases {
            let tokens = tokenize(input.to_string()).unwrap();
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0], expected_token);
        }
    }

    #[test]
    fn test_special_chars() {
        // Test cases for all possible characters
        let test_cases = vec![
            (
                ".",
                Token::Char {
                    val: Char::Dot,
                    pos: 0,
                },
            ),
            (
                "*",
                Token::Quantifier {
                    val: Quantifier::Any,
                    pos: 0,
                },
            ),
            (
                "+",
                Token::Quantifier {
                    val: Quantifier::Many,
                    pos: 0,
                },
            ),
            (
                "?",
                Token::Quantifier {
                    val: Quantifier::Maybe,
                    pos: 0,
                },
            ),
            (
                "^",
                Token::Anchor {
                    val: Anchor::Start,
                    pos: 0,
                },
            ),
            (
                "$",
                Token::Anchor {
                    val: Anchor::End,
                    pos: 0,
                },
            ),
            (
                "[",
                Token::Anchor {
                    val: Anchor::CharClassStart,
                    pos: 0,
                },
            ),
            (
                "]",
                Token::Anchor {
                    val: Anchor::CharClassEnd,
                    pos: 0,
                },
            ),
        ];

        for (input, expected_token) in test_cases {
            let tokens = tokenize(input.to_string()).unwrap();
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0], expected_token);
        }
    }
}
