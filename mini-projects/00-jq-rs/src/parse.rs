use crate::filter::FilterFn;
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Please enter a valid filter string")]
    InvalidString,
}

pub struct Parse {}

impl Parse {
    pub fn new() -> Parse {
        Parse {}
    }

    // Parse the input string into a sequence of filters
    pub fn parse(&self, input: &str) -> Result<Vec<FilterFn>, ParseError> {
        let mut filters: Vec<FilterFn> = Vec::new();

        let input = input.trim();

        // Handle pipes
        let pipe_regex = Regex::new(r"\|").unwrap();
        if pipe_regex.is_match(input) {
            let filter_strings: Vec<&str> = input.split('|').collect();
            let mut pipe_filters: Vec<FilterFn> = Vec::new();
            for filter_str in filter_strings {
                let parsed_filters = self.parse(filter_str.trim())?;
                pipe_filters.extend(parsed_filters);
            }
            return Ok(pipe_filters);
        }

        // Identity filter: Just a dot "."
        if input == "." {
            filters.push(FilterFn::Identity);
            return Ok(filters);
        }

        // Object-Identifier Index: .key
        let object_index_regex = Regex::new(r"^\.\w+$").unwrap();
        if object_index_regex.is_match(input) {
            let key = input.trim_start_matches('.').to_string();
            filters.push(FilterFn::KeyFilter(key));
            return Ok(filters);
        }

        // Array Index: .[index]
        let array_index_regex = Regex::new(r"^\.\[(\d+)\]$").unwrap();
        if let Some(caps) = array_index_regex.captures(input) {
            let index: usize = caps[1].parse().unwrap();
            filters.push(FilterFn::ArrayIndex(index));
            return Ok(filters);
        }

        // Array Slice: .[start:end]
        let array_slice_regex = Regex::new(r"^\.\[(\d*):(\d*)\]$").unwrap();
        if let Some(caps) = array_slice_regex.captures(input) {
            let start = caps
                .get(1)
                .map_or(0, |m| m.as_str().parse::<usize>().unwrap_or(0));
            let end = caps.get(2).and_then(|m| m.as_str().parse::<usize>().ok());
            filters.push(FilterFn::ArraySlice { start, end });
            return Ok(filters);
        }

        // Array Iterator: .[]
        if input == ".[]" {
            filters.push(FilterFn::ArrayIterator);
            return Ok(filters);
        }

        // Built-in functions
        if input == "add" {
            filters.push(FilterFn::Add);
            return Ok(filters);
        }

        if input == "length" {
            filters.push(FilterFn::Length);
            return Ok(filters);
        }

        // Del function: del(...)
        let del_regex = Regex::new(r"^del\((.+)\)$").unwrap();
        if let Some(caps) = del_regex.captures(input) {
            let arg = caps.get(1).unwrap().as_str().to_string();
            let arg_filters = self.parse(&arg)?;
            if arg_filters.len() != 1 {
                return Err(ParseError::InvalidString);
            }
            filters.push(FilterFn::Del(Box::new(arg_filters[0].clone())));
            return Ok(filters);
        }

        Err(ParseError::InvalidString)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::FilterFn;

    #[test]
    fn test_parse_identity() {
        let parser = Parse::new();
        let input = ".".to_string();
        let filters = parser.parse(&input).unwrap();
        assert_eq!(filters.len(), 1);
        match filters[0] {
            FilterFn::Identity => {}
            _ => panic!("Expected Identity filter"),
        }
    }

    #[test]
    fn test_parse_key_filter() {
        let parser = Parse::new();
        let input = ".key".to_string();
        let filters = parser.parse(&input).unwrap();
        assert_eq!(filters.len(), 1);
        match &filters[0] {
            FilterFn::KeyFilter(key) => assert_eq!(key, "key"),
            _ => panic!("Expected KeyFilter"),
        }
    }

    #[test]
    fn test_parse_array_index() {
        let parser = Parse::new();
        let input = ".[2]".to_string();
        let filters = parser.parse(&input).unwrap();
        assert_eq!(filters.len(), 1);
        match filters[0] {
            FilterFn::ArrayIndex(index) => assert_eq!(index, 2),
            _ => panic!("Expected ArrayIndex"),
        }
    }

    #[test]
    fn test_parse_array_slice() {
        let parser = Parse::new();
        let input = ".[1:3]".to_string();
        let filters = parser.parse(&input).unwrap();
        assert_eq!(filters.len(), 1);
        match filters[0] {
            FilterFn::ArraySlice { start, end } => {
                assert_eq!(start, 1);
                assert_eq!(end, Some(3));
            }
            _ => panic!("Expected ArraySlice"),
        }
    }

    #[test]
    fn test_parse_array_iterator() {
        let parser = Parse::new();
        let input = ".[]".to_string();
        let filters = parser.parse(&input).unwrap();
        assert_eq!(filters.len(), 1);
        match filters[0] {
            FilterFn::ArrayIterator => {}
            _ => panic!("Expected ArrayIterator"),
        }
    }

    #[test]
    fn test_parse_add_function() {
        let parser = Parse::new();
        let input = "add".to_string();
        let filters = parser.parse(&input).unwrap();
        assert_eq!(filters.len(), 1);
        match filters[0] {
            FilterFn::Add => {}
            _ => panic!("Expected Add function"),
        }
    }

    #[test]
    fn test_parse_length_function() {
        let parser = Parse::new();
        let input = "length".to_string();
        let filters = parser.parse(&input).unwrap();
        assert_eq!(filters.len(), 1);
        match filters[0] {
            FilterFn::Length => {}
            _ => panic!("Expected Length function"),
        }
    }

    #[test]
    fn test_parse_del_function_key() {
        let parser = Parse::new();
        let input = "del(.key)".to_string();
        let filters = parser.parse(&input).unwrap();
        assert_eq!(filters.len(), 1);
        match &filters[0] {
            FilterFn::Del(target) => match &**target {
                FilterFn::KeyFilter(key) => assert_eq!(key, "key"),
                _ => panic!("Expected KeyFilter inside Del"),
            },
            _ => panic!("Expected Del function"),
        }
    }

    #[test]
    fn test_parse_del_function_array_index() {
        let parser = Parse::new();
        let input = "del(.[1])".to_string();
        let filters = parser.parse(&input).unwrap();
        assert_eq!(filters.len(), 1);
        match &filters[0] {
            FilterFn::Del(target) => match &**target {
                FilterFn::ArrayIndex(index) => assert_eq!(*index, 1),
                _ => panic!("Expected ArrayIndex inside Del"),
            },
            _ => panic!("Expected Del function"),
        }
    }

    #[test]
    fn test_parse_pipe_operator() {
        let parser = Parse::new();
        let input = ".key | .[0] | .name".to_string();
        let filters = parser.parse(&input).unwrap();
        assert_eq!(filters.len(), 3);
        match &filters[0] {
            FilterFn::KeyFilter(key) => assert_eq!(key, "key"),
            _ => panic!("Expected KeyFilter"),
        }
        match filters[1] {
            FilterFn::ArrayIndex(index) => assert_eq!(index, 0),
            _ => panic!("Expected ArrayIndex"),
        }
        match &filters[2] {
            FilterFn::KeyFilter(key) => assert_eq!(key, "name"),
            _ => panic!("Expected KeyFilter"),
        }
    }

    #[test]
    fn test_parse_invalid_input() {
        let parser = Parse::new();
        let input = "invalid_filter".to_string();
        let result = parser.parse(&input);
        assert!(result.is_err());
    }
}
