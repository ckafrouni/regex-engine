use super::errors;
use super::tokenizer;

#[derive(Debug)]
pub enum AstNode {
    ChainNode(Vec<Box<AstNode>>),
    QuantifierNode(tokenizer::Quantifier, Box<AstNode>),
    CharNode(tokenizer::Char),
}

pub fn parse(tokens: Vec<tokenizer::Token>) -> Result<AstNode, errors::ParseError> {
    let mut chain: Vec<Box<AstNode>> = vec![];
    let mut it = tokens.iter().peekable();
    while let Some(tk) = it.next() {
        let node: AstNode = match tk {
            tokenizer::Token::Char(c) => AstNode::CharNode(*c),
            _ => {
                return Err(errors::ParseError::UnexpectedToken(
                    "Should have found a char literal".into(),
                ))
            }
        };

        match it.peek() {
            Some(tokenizer::Token::Quantifier(m)) => {
                it.next();
                chain.push(Box::new(AstNode::QuantifierNode(*m, Box::new(node))))
            }
            _ => chain.push(Box::new(node)),
        }
    }

    let root = AstNode::ChainNode(chain);
    Ok(root)
}
