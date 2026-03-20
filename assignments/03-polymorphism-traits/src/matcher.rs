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
    fn test_regex_matcher_simple_match() {
        let matcher = RegexMatcher::new("foo", false).unwrap();
        assert!(matcher.is_match("foo"));
        assert!(!matcher.is_match("bar"));
    }

    #[test]
    fn test_regex_matcher_character_class() {
        // Matches any character between 'a' and 'c'
        let matcher = RegexMatcher::new("[a-c]", false).unwrap();
        assert!(matcher.is_match("a"));
        assert!(matcher.is_match("b"));
        assert!(matcher.is_match("c"));
        assert!(!matcher.is_match("d"));
    }

    #[test]
    fn test_regex_matcher_quantifiers() {
        // Matches 'a' followed by zero or more 'b's
        let matcher = RegexMatcher::new("ab*", false).unwrap();
        assert!(matcher.is_match("a"));
        assert!(matcher.is_match("ab"));
        assert!(matcher.is_match("abb"));
        assert!(matcher.is_match("abbbbb"));
        assert!(!matcher.is_match("b"));
    }

    #[test]
    fn test_regex_matcher_groups_and_alternation() {
        // Matches 'cat' or 'dog'
        let matcher = RegexMatcher::new("(cat|dog)", false).unwrap();
        assert!(matcher.is_match("I have a cat"));
        assert!(matcher.is_match("I have a dog"));
        assert!(!matcher.is_match("I have a mouse"));
    }

    #[test]
    fn test_regex_matcher_special_characters() {
        // Matches strings containing a dot '.'
        let matcher = RegexMatcher::new(r"\.", false).unwrap();
        assert!(matcher.is_match("file.txt"));
        assert!(matcher.is_match("a.b"));
        assert!(!matcher.is_match("abc"));
    }

    #[test]
    fn test_regex_matcher_digit_class() {
        // Matches any string containing digits
        let matcher = RegexMatcher::new(r"\d+", false).unwrap();
        assert!(matcher.is_match("123"));
        assert!(matcher.is_match("abc123"));
        assert!(!matcher.is_match("abc"));
    }

    #[test]
    fn test_regex_matcher_invalid_pattern() {
        assert!(RegexMatcher::new("*", false).is_err());
        assert!(RegexMatcher::new("[unclosed", false).is_err());
    }
}
