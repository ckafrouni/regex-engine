pub mod errors;
mod parser;
mod regex;
mod tokenizer;

pub use regex::Regex;

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
    fn test_match() {
        let reg = Regex::new(r#"hel.*"#).unwrap();

        // Should match
        let matched = reg.find("  heloooo");
        println!("Matched: {:?}", matched);
        assert!(matched.is_match());

        let matched = reg.find("helpsdf");
        println!("Matched: {:?}", matched);
        assert!(matched.is_match());

        let matched = reg.find("hel.q");
        println!("Matched: {:?}", matched);
        assert!(matched.is_match());

        let matched = reg.find("hel sdf");
        println!("Matched: {:?}", matched);
        assert!(matched.is_match());

        // Should not match
        let matched = reg.find("he");
        println!("Matched: {:?}", matched);
        assert!(!matched.is_match());
    }
}
