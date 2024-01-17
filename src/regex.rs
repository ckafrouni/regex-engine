use super::{errors, parser, tokenizer};

#[derive(Default, Debug)]
pub struct Match {
    is_match: bool,
    matched: Option<String>,
    start: usize,
    end: usize,
}

impl Match {
    pub fn range(&self) -> (usize, usize) {
        (self.start, self.end)
    }

    pub fn matched(&self) -> Option<&str> {
        self.matched.as_deref()
    }

    pub fn is_match(&self) -> bool {
        self.is_match
    }
}

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
    pub fn find(&self, s: impl AsRef<str>) -> Match {
        let s = s.as_ref();
        let mut match_res = Match::default();

        match &self.ast {
            parser::AstNode::StartAnchor(ast) => {
                if let Some(match_len) = self.match_node(ast, s) {
                    match_res.is_match = true;
                    match_res.start = 0;
                    match_res.end = match_len;
                    match_res.matched = Some(s[..match_len].into());
                }
            }
            _ => {
                for (i, _) in s.char_indices() {
                    if let Some(match_len) = self.match_node(&self.ast, &s[i..]) {
                        match_res.is_match = true;
                        match_res.start = i;
                        match_res.end = i + match_len;
                        match_res.matched = Some(s[i..i + match_len].into());
                        break;
                    }
                }
            }
        }

        match_res
    }

    fn match_node(&self, node: &parser::AstNode, s: &str) -> Option<usize> {
        match node {
            parser::AstNode::Char(tokenizer::Char::Lit(c)) => {
                if s.starts_with(*c) {
                    Some(1)
                } else {
                    None
                }
            }
            parser::AstNode::Char(tokenizer::Char::Dot) => {
                if !s.is_empty() {
                    Some(1)
                } else {
                    None
                }
            } // Matches any single character
            parser::AstNode::Chain(nodes) => self.match_chain(nodes, s),
            parser::AstNode::Quantifier(q, n) => self.match_quantifier(*q, n, s),
            parser::AstNode::EndAnchor => {
                if s.is_empty() {
                    Some(0)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn match_chain(&self, nodes: &[parser::AstNode], s: &str) -> Option<usize> {
        let mut current_str = s;
        let mut total_matched = 0;

        for node in nodes.iter() {
            match self.match_node(node, current_str) {
                Some(match_len) => {
                    total_matched += match_len;
                    current_str = &current_str[match_len..];
                }
                None => return None,
            }
        }

        Some(total_matched)
    }

    fn match_quantifier(
        &self,
        quantifier: tokenizer::Quantifier,
        node: &parser::AstNode,
        s: &str,
    ) -> Option<usize> {
        match quantifier {
            tokenizer::Quantifier::Any => self.match_any(node, s),
            tokenizer::Quantifier::Many => self.match_many(node, s),
            tokenizer::Quantifier::Maybe => self.match_option(node, s),
        }
    }

    fn match_any(&self, node: &parser::AstNode, s: &str) -> Option<usize> {
        // Zero or more
        let mut current_str = s;
        let mut count = 0_usize;

        while let Some(n) = self.match_node(node, current_str) {
            count += n;
            current_str = &current_str[n..];
        }

        Some(count)
    }

    fn match_many(&self, node: &parser::AstNode, s: &str) -> Option<usize> {
        // One or more
        match self.match_node(node, s) {
            Some(first_len) => {
                let rest_len = self.match_any(node, &s[first_len..]).unwrap_or(0);
                Some(first_len + rest_len)
            }
            None => None,
        }
    }

    fn match_option(&self, node: &parser::AstNode, s: &str) -> Option<usize> {
        // Zero or one
        self.match_node(node, s).or(Some(0))
    }
}
