use crate::tokenizer::{Anchor, Char, Quantifier, Token};

use super::errors::ParseError;

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Chain(Box<[AstNode]>),
    Quantifier(Quantifier, Box<AstNode>),
    Char(Char),
    CharClass(Box<[Char]>),
    StartAnchor(Box<AstNode>),
    EndAnchor,
    CaptureGroup(Box<[AstNode]>),
}

pub fn parse(tokens: Vec<Token>) -> Result<AstNode, ParseError> {
    let mut chain: Vec<AstNode> = vec![];
    let mut has_start_anchor = false;
    let mut has_end_anchor = false;
    let mut tokens = tokens.iter().peekable();

    if let Some(Token::Anchor {
        pos: _,
        val: Anchor::Start,
    }) = tokens.peek()
    {
        tokens.next();
        has_start_anchor = true;
    }

    while let Some(tok) = tokens.next() {
        if has_end_anchor {
            return Err(ParseError::UnexpectedToken(
                Token::Anchor {
                    pos: tok.pos(),
                    val: Anchor::End,
                },
                "End anchor only allowed at the end of the string".into(),
            ));
        }

        let node: AstNode = match tok {
            Token::Char { val, .. } => AstNode::Char(*val),
            Token::Anchor {
                val: Anchor::CharClassStart { .. },
                ..
            } => {
                let mut chars = vec![];
                for tok in tokens.by_ref() {
                    match tok {
                        Token::Char { val, .. } => chars.push(*val),
                        Token::Anchor {
                            val: Anchor::CharClassEnd { .. },
                            ..
                        } => break,
                        _ => {
                            return Err(ParseError::UnexpectedToken(
                                *tok,
                                "Should have found a char literal".into(),
                            ))
                        }
                    }
                }
                AstNode::CharClass(chars.into())
            }
            Token::Anchor {
                val: Anchor::CharClassEnd { .. },
                ..
            } => {
                return Err(ParseError::UnexpectedToken(
                    *tok,
                    "Char class end anchor only allowed after char class start anchor".into(),
                ))
            }
            Token::Anchor {
                val: Anchor::GroupStart { .. },
                ..
            } => {
                let mut group = vec![];
                for tok in tokens.by_ref() {
                    match tok {
                        Token::Char { val, .. } => group.push(AstNode::Char(*val)),
                        Token::Anchor {
                            val: Anchor::GroupEnd { .. },
                            ..
                        } => break,
                        _ => {
                            return Err(ParseError::UnexpectedToken(
                                *tok,
                                "Should have found a char literal".into(),
                            ))
                        }
                    }
                }
                AstNode::CaptureGroup(group.into())
            }
            Token::Anchor {
                val: Anchor::GroupEnd { .. },
                ..
            } => {
                return Err(ParseError::UnexpectedToken(
                    *tok,
                    "Group end anchor only allowed after group start anchor".into(),
                ))
            }
            Token::Anchor {
                val: Anchor::Start, ..
            } => {
                return Err(ParseError::UnexpectedToken(
                    *tok,
                    "Start anchor only allowed at the start of the string".into(),
                ))
            }
            Token::Anchor {
                val: Anchor::End, ..
            } => {
                has_end_anchor = true;
                AstNode::EndAnchor
            }
            _ => {
                return Err(ParseError::UnexpectedToken(
                    *tok,
                    "Should have found a char literal".into(),
                ))
            }
        };

        match tokens.peek() {
            Some(&Token::Quantifier { val, .. }) => {
                tokens.next();
                chain.push(AstNode::Quantifier(*val, Box::new(node)))
            }
            _ => chain.push(node),
        }
    }

    if has_start_anchor {
        Ok(AstNode::StartAnchor(Box::new(AstNode::Chain(chain.into()))))
    } else {
        Ok(AstNode::Chain(chain.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::tokenize;

    #[test]
    fn test_parse_char_class() {
        let tokens = tokenize("[abc]".into()).unwrap();

        let expected_ast = AstNode::Chain(
            vec![AstNode::CharClass(
                vec![Char::Lit('a'), Char::Lit('b'), Char::Lit('c')].into(),
            )]
            .into(),
        );

        let ast = parse(tokens).unwrap();
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn test_capture_group() {
        let tokens = tokenize("(abc)".into()).unwrap();

        let expected_ast = AstNode::Chain(
            vec![AstNode::CaptureGroup(
                vec![
                    AstNode::Char(Char::Lit('a')),
                    AstNode::Char(Char::Lit('b')),
                    AstNode::Char(Char::Lit('c')),
                ]
                .into(),
            )]
            .into(),
        );

        let ast = parse(tokens).unwrap();
        // println!("{ast:#?}");
        assert_eq!(ast, expected_ast);
    }
}
