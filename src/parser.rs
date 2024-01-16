use super::errors;
use super::tokenizer;

// #[derive(Debug)]
// pub enum Ast {
//     StartAnchor(Box<AstNode >)
// }

#[derive(Debug)]
pub enum AstNode {
    Chain(Vec<Box<AstNode>>),
    Quantifier(tokenizer::Quantifier, Box<AstNode>),
    Char(tokenizer::Char),
    StartAnchor(Box<AstNode>),
    EndAnchor,
}

pub fn parse(tokens: Vec<tokenizer::Token>) -> Result<AstNode, errors::ParseError> {
    let mut chain: Vec<Box<AstNode>> = vec![];
    let mut has_start_anchor = false;
    let mut has_end_anchor = false;
    let mut tokens = tokens.iter().peekable();

    if let Some(tokenizer::Token::Anchor(tokenizer::Anchor::Start)) = tokens.peek() {
        tokens.next();
        has_start_anchor = true;
    }

    while let Some(tok) = tokens.next() {
        if has_end_anchor {
            return Err(errors::ParseError::UnexpectedToken(
                "End anchor only allowed at the end of the string".into(),
            ));
        }
        let node: AstNode = match tok {
            tokenizer::Token::Char(c) => AstNode::Char(*c),
            tokenizer::Token::Anchor(tokenizer::Anchor::Start) => {
                return Err(errors::ParseError::UnexpectedToken(
                    "Start anchor only allowed at the start of the string".into(),
                ))
            }
            tokenizer::Token::Anchor(tokenizer::Anchor::End) => {
                has_end_anchor = true;
                AstNode::EndAnchor
            }
            _ => {
                return Err(errors::ParseError::UnexpectedToken(
                    "Should have found a char literal".into(),
                ))
            }
        };

        match tokens.peek() {
            Some(tokenizer::Token::Quantifier(m)) => {
                tokens.next();
                chain.push(Box::new(AstNode::Quantifier(*m, Box::new(node))))
            }
            _ => chain.push(Box::new(node)),
        }
    }

    if has_start_anchor {
        Ok(AstNode::StartAnchor(Box::new(AstNode::Chain(chain))))
    } else {
        Ok(AstNode::Chain(chain))
    }
}
