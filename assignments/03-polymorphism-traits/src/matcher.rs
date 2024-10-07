// src/matcher.rs
use regex::Regex;

pub trait Matcher {
    fn is_match(&self, line: &str) -> bool;
}

pub struct LiteralMatcher {
    needle: String,
    ignore_case: bool,
}

impl LiteralMatcher {
    pub fn new(needle: String, ignore_case: bool) -> Self {
        LiteralMatcher {
            needle,
            ignore_case,
        }
    }
}

impl Matcher for LiteralMatcher {
    fn is_match(&self, line: &str) -> bool {
        if self.ignore_case {
            line.to_lowercase().contains(&self.needle.to_lowercase())
        } else {
            line.contains(&self.needle)
        }
    }
}

pub struct RegexMatcher {
    regex: Regex,
}

impl RegexMatcher {
    pub fn new(pattern: &str, ignore_case: bool) -> Result<Self, regex::Error> {
        let regex_pattern = if ignore_case {
            format!("(?i){}", pattern) // `(?i)` makes regex case-insensitive
        } else {
            pattern.to_string()
        };
        let regex = Regex::new(&regex_pattern)?;
        Ok(RegexMatcher { regex })
    }
}

impl Matcher for RegexMatcher {
    fn is_match(&self, line: &str) -> bool {
        self.regex.is_match(line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_matcher() {
        let matcher = LiteralMatcher::new("foo".to_string(), false);
        assert!(matcher.is_match("foo"));
        assert!(!matcher.is_match("bar"));

        let matcher = LiteralMatcher::new("foo".to_string(), true);
        assert!(matcher.is_match("FOO"));
        assert!(!matcher.is_match("bar"));
    }

    #[test]
    fn test_regex_matcher() {
        let matcher = RegexMatcher::new("foo", false).unwrap();
        assert!(matcher.is_match("foo"));
        assert!(!matcher.is_match("bar"));
    }

    #[test]
    fn test_regex_matcher_case_insensitive() {
        let matcher = RegexMatcher::new("foo", true).unwrap();
        assert!(matcher.is_match("FOO"));
        assert!(!matcher.is_match("bar"));
    }

    #[test]
    fn test_regex_matcher_case_sensitive() {
        let matcher = RegexMatcher::new("foo", false).unwrap();
        assert!(!matcher.is_match("FOO"));
        assert!(!matcher.is_match("bar"));
    }

    #[test]
    fn test_regex_matcher_invalid_pattern() {
        assert!(RegexMatcher::new("*", false).is_err());
    }
}
