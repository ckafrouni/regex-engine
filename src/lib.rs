pub mod errors;
mod parser;
mod regex;
mod tokenizer;

pub use regex::Regex;

// DONE
// // TODO : Add support for Anchors. Modify and finilize the Ast.
// // I think we should have the Anchor::Start enum take the rest of the ast
// // in its attribute instead of adding the anchor to the list.
// // ex: StartAnchor(Box<AstNode>)

// TODO : is_match() -> bool

// TODO : find() -> Match
// Match can be a structure with :
//      - .matched (bool)
//      - .start (index)
//      - .end (index)
//      - range() (range (start, end) index of what was matched)

// TODO : [] character classes
// TODO : non-capturing groups (?:abc)*
// TODO : capturing groups (abc)
// TODO : add support for "abc*d(efg)+" parens
// TODO : later support captures

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_start_anchor() {
        let reg = Regex::new(r#"hello"#).unwrap();

        assert!(reg.is_match("hello"));
        assert!(reg.is_match("hello "));
        assert!(reg.is_match("hello world"));
        assert!(reg.is_match("  hellow"));

        assert!(!reg.is_match("hell"));
    }

    #[test]
    fn test_start_anchor() {
        let reg = Regex::new(r#"^hello"#).unwrap();

        assert!(reg.is_match("hello"));
        assert!(!reg.is_match("qhello"));
    }

    #[test]
    fn test_end_anchor() {
        let reg = Regex::new(r#"hello$"#).unwrap();

        assert!(reg.is_match("hello"));
        assert!(!reg.is_match("helloq"));
    }

    #[test]
    fn test_any() {
        let reg = Regex::new(r#"he*llo"#).unwrap();

        assert!(reg.is_match("hello"));
        assert!(reg.is_match("heeello"));
        assert!(reg.is_match("hllo"));
    }

    #[test]
    fn test_many() {
        let reg = Regex::new(r#"he+llo"#).unwrap();

        assert!(reg.is_match("hello"));
        assert!(reg.is_match("heeello"));

        assert!(!reg.is_match("hllo"));
    }

    #[test]
    fn test_maybe() {
        let reg = Regex::new(r#"he?llo"#).unwrap();

        assert!(reg.is_match("hello"));
        assert!(reg.is_match("hllo"));

        assert!(!reg.is_match("llo"));
        assert!(!reg.is_match("hlo"));
        assert!(!reg.is_match("heeello"));
    }

    #[test]
    fn test_dot() {
        let reg = Regex::new(r#"hel."#).unwrap();

        assert!(reg.is_match("helo"));
        assert!(reg.is_match("help"));
        assert!(reg.is_match("hel."));
        assert!(reg.is_match("hel "));
    }

    #[test]
    fn test_dot_any() {
        let reg = Regex::new(r#"hel.*"#).unwrap();

        assert!(reg.is_match("heloooo"));
        assert!(reg.is_match("helpsdf"));
        assert!(reg.is_match("hel.q"));
        assert!(reg.is_match("hel sdf"));
    }
}
