use super::errors;
use super::tokenizer;

#[derive(Debug)]
pub enum AstNode {
    Chain(Vec<Box<AstNode>>),
    Quantifier(tokenizer::Quantifier, Box<AstNode>),
    Char(tokenizer::Char),
    Anchor(tokenizer::Anchor),
}

pub fn parse(tokens: Vec<tokenizer::Token>) -> Result<AstNode, errors::ParseError> {
    let mut chain: Vec<Box<AstNode>> = vec![];
    let mut it = tokens.iter().peekable();
    while let Some(tk) = it.next() {
        let node: AstNode = match tk {
            tokenizer::Token::Char(c) => AstNode::Char(*c),
            tokenizer::Token::Anchor(tokenizer::Anchor::Start) => {
                if !chain.is_empty() {
                    return Err(errors::ParseError::UnexpectedToken(
                        "Start anchor token not at the start".into(),
                    ));
                }
                AstNode::Anchor(tokenizer::Anchor::Start)
            }
            tokenizer::Token::Anchor(tokenizer::Anchor::End) => {
                if it.peek().is_some() {
                    return Err(errors::ParseError::UnexpectedToken(
                        "End anchor token not at the end".into(),
                    ));
                }
                AstNode::Anchor(tokenizer::Anchor::End)
            }
            _ => {
                return Err(errors::ParseError::UnexpectedToken(
                    "Should have found a char literal".into(),
                ))
            }
        };

        match it.peek() {
            Some(tokenizer::Token::Quantifier(m)) => {
                it.next();
                chain.push(Box::new(AstNode::Quantifier(*m, Box::new(node))))
            }
            _ => chain.push(Box::new(node)),
        }
    }

    let root = AstNode::Chain(chain);
    Ok(root)
}
