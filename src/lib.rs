pub mod errors;
mod parser;
mod regex;
mod tokenizer;

pub use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let reg = Regex::new(r#"hello"#).unwrap();

        assert!(reg.is_match("hello"));
        assert!(reg.is_match("hello "));
        assert!(reg.is_match("hello world"));
        assert!(reg.is_match("hellow"));

        assert!(!reg.is_match("hell"));
    }

    #[test]
    fn test_empty_start() {
        let reg = Regex::new(r#"^hello"#).unwrap();

        assert!(reg.is_match("hello"));
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
