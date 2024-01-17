use crate::tokenizer::{Anchor, Char, Quantifier, Token};

use super::errors::ParseError;

#[derive(Debug)]
pub enum AstNode {
    Chain(Box<[AstNode]>),
    Quantifier(Quantifier, Box<AstNode>),
    Char(Char),
    StartAnchor(Box<AstNode>),
    EndAnchor,
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
