pub mod errors;
mod parser;
mod tokenizer;

#[derive(Debug)]
pub struct Regex {
    ast: parser::AstNode,
}

impl Regex {
    pub fn new(pattern: impl Into<String>) -> Result<Self, errors::ParseError> {
        let tokens = tokenizer::tokenize(pattern.into())?;
        let ast = parser::parse(tokens)?;
        Ok(Self { ast })
    }
}

impl Regex {
    pub fn is_match(&self, s: impl AsRef<str>) -> bool {
        self.match_node(&self.ast, s.as_ref())
    }

    fn match_node(&self, node: &parser::AstNode, s: &str) -> bool {
        match node {
            parser::AstNode::CharNode(tokenizer::Char::Lit(c)) => s.starts_with(*c),
            parser::AstNode::CharNode(tokenizer::Char::Dot) => !s.is_empty(), // Matches any single character
            parser::AstNode::ChainNode(nodes) => self.match_chain(nodes, s),
            parser::AstNode::QuantifierNode(q, n) => self.match_quantifier(*q, n, s),
        }
    }

    fn match_chain(&self, nodes: &[Box<parser::AstNode>], s: &str) -> bool {
        let mut current_str = s;

        for node in nodes {
            if !self.match_node(node, current_str) {
                return false;
            }

            // Move the current_str ahead by the length of the match
            // This logic may vary based on how you want to handle the matches
            let match_len = current_str.chars().next().map_or(0, |_| 1);
            current_str = &current_str[match_len..];
        }

        current_str.is_empty()
    }

    fn match_quantifier(
        &self,
        quantifier: tokenizer::Quantifier,
        node: &Box<parser::AstNode>,
        s: &str,
    ) -> bool {
        match quantifier {
            tokenizer::Quantifier::Any => self.match_any(node, s),
            tokenizer::Quantifier::Many => self.match_many(node, s),
            tokenizer::Quantifier::Option => self.match_option(node, s),
        }
    }

    fn match_any(&self, node: &Box<parser::AstNode>, s: &str) -> bool {
        // Zero or more
        let mut current_str = s;

        while self.match_node(node, current_str) {
            let match_len = current_str.chars().next().map_or(0, |_| 1);
            current_str = &current_str[match_len..];
        }

        true
    }

    fn match_many(&self, node: &Box<parser::AstNode>, s: &str) -> bool {
        // One or more
        if !self.match_node(node, s) {
            return false;
        }

        self.match_any(node, s)
    }

    fn match_option(&self, node: &Box<parser::AstNode>, s: &str) -> bool {
        // Zero or one
        self.match_node(node, s) || true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let reg = Regex::new(r#"he*llo"#).unwrap();

        println!("{:#?}", reg);

        assert!(reg.is_match("hello"));
        assert!(reg.is_match("heeello"));
        assert!(reg.is_match("hllo"));
    }
}
