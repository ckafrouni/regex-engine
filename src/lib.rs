//! TODO : non-capturing groups (?:abc)*
//!
//! TODO : capturing groups (abc) - started
//!
//! TODO : add support for "abc*d(efg)+" parens
//!
//! TODO : later support captures

pub mod errors;
mod parser;
mod regex;
mod tokenizer;

pub use regex::Match;
pub use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_readme() {
        let regex = Regex::new("^[hH]ello,? [wW]orld ?!").unwrap();
        let res = regex.find("Hello, world!");
        assert!(res.is_match());
    }

    #[test]
    fn test_no_start_anchor() {
        let reg = Regex::new(r#"hello"#).unwrap();

        assert!(reg.find("hello").is_match());
        assert!(reg.find("hello ").is_match());
        assert!(reg.find("hello world").is_match());
        assert!(reg.find("  hellow").is_match());

        assert!(!reg.find("hell").is_match());
    }

    #[test]
    fn test_start_anchor() {
        let reg = Regex::new(r#"^hello"#).unwrap();

        assert!(reg.find("hello").is_match());
        assert!(!reg.find("qhello").is_match());
    }

    #[test]
    fn test_end_anchor() {
        let reg = Regex::new(r#"hello$"#).unwrap();

        assert!(reg.find("hello").is_match());
        assert!(!reg.find("helloq").is_match());
    }

    #[test]
    fn test_any() {
        let reg = Regex::new(r#"he*llo"#).unwrap();

        assert!(reg.find("hello").is_match());
        assert!(reg.find("heeello").is_match());
        assert!(reg.find("hllo").is_match());
    }

    #[test]
    fn test_many() {
        let reg = Regex::new(r#"he+llo"#).unwrap();

        assert!(reg.find("hello").is_match());
        assert!(reg.find("heeello").is_match());

        assert!(!reg.find("hllo").is_match());
    }

    #[test]
    fn test_maybe() {
        let reg = Regex::new(r#"he?llo"#).unwrap();

        assert!(reg.find("hello").is_match());
        assert!(reg.find("hllo").is_match());

        assert!(!reg.find("llo").is_match());
        assert!(!reg.find("hlo").is_match());
        assert!(!reg.find("heeello").is_match());
    }

    #[test]
    fn test_dot() {
        let reg = Regex::new(r#"hel."#).unwrap();

        assert!(reg.find("helo").is_match());
        assert!(reg.find("help").is_match());
        assert!(reg.find("hel.").is_match());
        assert!(reg.find("hel ").is_match());
    }

    #[test]
    fn test_dot_any() {
        let reg = Regex::new(r#"hel.*"#).unwrap();

        assert!(reg.find("heloooo").is_match());
        assert!(reg.find("helpsdf").is_match());
        assert!(reg.find("hel.q").is_match());
        assert!(reg.find("hel sdf").is_match());
    }

    #[test]
    fn test_dot_many() {
        let reg = Regex::new(r#"hel.+"#).unwrap();

        assert!(reg.find("heloooo").is_match());
        assert!(reg.find("helpsdf").is_match());
        assert!(reg.find("hel.q").is_match());

        assert!(!reg.find("hel").is_match());
    }

    #[test]
    fn test_dot_maybe() {
        let reg = Regex::new(r#"hel.?"#).unwrap();

        assert!(reg.find("heloooo").is_match());
        assert!(reg.find("helpsdf").is_match());
        assert!(reg.find("hel.q").is_match());
        assert!(reg.find("hel").is_match());
    }

    #[test]
    fn test_char_class() {
        let reg = Regex::new(r#"[abc]"#).unwrap();

        assert!(reg.find("a").is_match());
        assert!(reg.find("b").is_match());
        assert!(reg.find("c").is_match());

        assert!(!reg.find("d").is_match());
        assert!(!reg.find("de").is_match());
    }

    #[test]
    fn test_char_class_any() {
        let reg = Regex::new(r#"[abc]*"#).unwrap();

        assert!(reg.find("a").is_match());
        assert!(reg.find("b").is_match());
        assert!(reg.find("c").is_match());
        assert!(reg.find("abc").is_match());
        assert!(reg.find("abccba").is_match());

        assert!(reg.find("d").is_match());
        assert!(reg.find("de").is_match());
    }

    #[test]
    fn test_char_class_start_anchor() {
        let reg = Regex::new(r#"^[abc]"#).unwrap();

        assert!(reg.find("a").is_match());
        assert!(reg.find("b").is_match());
        assert!(reg.find("c").is_match());

        assert!(!reg.find("d").is_match());
        assert!(!reg.find(" a").is_match());
    }
}
