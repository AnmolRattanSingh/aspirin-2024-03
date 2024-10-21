use serde_json::Value;
use thiserror::Error;

// Define custom error types using `thiserror`
#[derive(Error, Debug)]
pub enum FilterError {
    #[error("Key '{0}' not found in object")]
    KeyNotFound(String),

    #[error("Index {0} out of bounds")]
    IndexOutOfBounds(usize),

    #[error("Expected an array but found something else")]
    ExpectedArray,

    #[error("Slice out of bounds")]
    SliceOutOfBounds,

    #[error("Invalid type encountered")]
    InvalidType,

    #[error("Deletion target not found or invalid")]
    DeletionError,
}

// Define the FilterFn enum
#[derive(Clone)]
pub enum FilterFn {
    Identity,
    KeyFilter(String),
    ArrayIndex(usize),
    ArraySlice { start: usize, end: Option<usize> },
    ArrayIterator,
    Add,
    Length,
    Del(Box<FilterFn>),
}

pub struct Filter {}

impl Filter {
    // Create a new filter instance
    pub fn new() -> Filter {
        Filter {}
    }

    // Identity filter: just returns the input as is
    pub fn identity_filter(&self, input: Value) -> Result<Value, FilterError> {
        Ok(input)
    }

    // Key filter: accessing a key in a JSON object
    pub fn key_filter(&self, input: Value, key: &str) -> Result<Value, FilterError> {
        match input {
            Value::Object(map) => match map.get(key) {
                Some(v) => Ok(v.clone()),
                None => Err(FilterError::KeyNotFound(key.to_string())),
            },
            _ => Err(FilterError::InvalidType),
        }
    }

    // Array index: accessing an index in a JSON array
    pub fn array_index(&self, input: Value, index: usize) -> Result<Value, FilterError> {
        match input {
            Value::Array(arr) => arr
                .get(index)
                .cloned()
                .ok_or_else(|| FilterError::IndexOutOfBounds(index)),
            _ => Err(FilterError::ExpectedArray),
        }
    }

    // Array slice: accessing a slice of a JSON array
    pub fn array_slice(
        &self,
        input: Value,
        start: usize,
        end: Option<usize>,
    ) -> Result<Value, FilterError> {
        match input {
            Value::Array(arr) => {
                let slice = match end {
                    Some(end_idx) => arr.get(start..end_idx),
                    None => arr.get(start..),
                };
                match slice {
                    Some(s) => Ok(Value::Array(s.to_vec())),
                    None => Err(FilterError::SliceOutOfBounds),
                }
            }
            _ => Err(FilterError::ExpectedArray),
        }
    }

    // Implement the 'add' function
    pub fn add(&self, input: Value) -> Result<Value, FilterError> {
        match input {
            Value::Array(arr) => {
                let mut sum_i64 = 0i64;
                let mut sum_f64 = 0.0f64;
                let mut concatenated_string = String::new();
                let mut has_number = false;
                let mut has_float = false;
                let mut has_string = false;

                for value in arr {
                    match value {
                        Value::Number(num) => {
                            if let Some(i) = num.as_i64() {
                                sum_i64 += i;
                                has_number = true;
                            } else if let Some(f) = num.as_f64() {
                                sum_f64 += f;
                                has_float = true;
                            }
                        }
                        Value::String(s) => {
                            concatenated_string.push_str(&s);
                            has_string = true;
                        }
                        _ => {
                            return Err(FilterError::InvalidType);
                        }
                    }
                }

                if has_string && !concatenated_string.is_empty() {
                    Ok(Value::String(concatenated_string))
                } else if has_float || has_number {
                    let total = sum_f64 + sum_i64 as f64;
                    Ok(Value::Number(serde_json::Number::from_f64(total).unwrap()))
                } else {
                    Err(FilterError::InvalidType)
                }
            }
            _ => Err(FilterError::ExpectedArray),
        }
    }

    // Implement the 'length' function
    pub fn length(&self, input: Value) -> Result<Value, FilterError> {
        match input {
            Value::Array(arr) => Ok(Value::Number(serde_json::Number::from(arr.len()))),
            Value::Object(map) => Ok(Value::Number(serde_json::Number::from(map.len()))),
            Value::String(s) => Ok(Value::Number(serde_json::Number::from(s.chars().count()))),
            _ => Err(FilterError::InvalidType),
        }
    }

    // Implement the 'del' function
    pub fn del(&self, input: Value, target: &FilterFn) -> Result<Value, FilterError> {
        match target {
            FilterFn::KeyFilter(key) => {
                if let Value::Object(mut map) = input {
                    map.remove(key);
                    Ok(Value::Object(map))
                } else {
                    Err(FilterError::InvalidType)
                }
            }
            FilterFn::ArrayIndex(index) => {
                if let Value::Array(mut arr) = input {
                    if *index < arr.len() {
                        arr.remove(*index);
                        Ok(Value::Array(arr))
                    } else {
                        Err(FilterError::IndexOutOfBounds(*index))
                    }
                } else {
                    Err(FilterError::ExpectedArray)
                }
            }
            _ => Err(FilterError::InvalidType),
        }
    }
}

// Implement the apply method for FilterFn
impl FilterFn {
    pub fn apply(&self, filter: &Filter, values: Vec<Value>) -> Result<Vec<Value>, FilterError> {
        let mut results = Vec::new();
        match self {
            FilterFn::Identity => {
                results.extend(values);
            }
            FilterFn::KeyFilter(key) => {
                for value in values {
                    let result = filter.key_filter(value, key)?;
                    results.push(result);
                }
            }
            FilterFn::ArrayIndex(index) => {
                for value in values {
                    let result = filter.array_index(value, *index)?;
                    results.push(result);
                }
            }
            FilterFn::ArraySlice { start, end } => {
                for value in values {
                    let result = filter.array_slice(value, *start, *end)?;
                    results.push(result);
                }
            }
            FilterFn::ArrayIterator => {
                for value in values {
                    match value {
                        Value::Array(arr) => {
                            results.extend(arr);
                        }
                        _ => return Err(FilterError::ExpectedArray),
                    }
                }
            }
            FilterFn::Add => {
                if values.len() != 1 {
                    return Err(FilterError::InvalidType);
                }
                let result = filter.add(values.into_iter().next().unwrap())?;
                results.push(result);
            }
            FilterFn::Length => {
                for value in values {
                    let result = filter.length(value)?;
                    results.push(result);
                }
            }
            FilterFn::Del(target) => {
                for value in values {
                    let result = filter.del(value, target)?;
                    results.push(result);
                }
            }
        }
        Ok(results)
    }
}

// tests/filter_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_identity_filter() {
        let filter = Filter::new();
        let input = json!({"key": "value"});
        let result = filter.identity_filter(input.clone()).unwrap();
        assert_eq!(result, input);
    }

    #[test]
    fn test_key_filter() {
        let filter = Filter::new();
        let input = json!({"key": "value", "other": "data"});
        let result = filter.key_filter(input.clone(), "key").unwrap();
        assert_eq!(result, json!("value"));
    }

    #[test]
    fn test_key_filter_nonexistent_key() {
        let filter = Filter::new();
        let input = json!({"key": "value"});
        let result = filter.key_filter(input, "nonexistent");
        assert!(matches!(result, Err(FilterError::KeyNotFound(_))));
    }

    #[test]
    fn test_array_index() {
        let filter = Filter::new();
        let input = json!(["zero", "one", "two"]);
        let result = filter.array_index(input.clone(), 1).unwrap();
        assert_eq!(result, json!("one"));
    }

    #[test]
    fn test_array_index_out_of_bounds() {
        let filter = Filter::new();
        let input = json!(["zero", "one", "two"]);
        let result = filter.array_index(input, 5);
        assert!(matches!(result, Err(FilterError::IndexOutOfBounds(_))));
    }

    #[test]
    fn test_array_slice() {
        let filter = Filter::new();
        let input = json!(["zero", "one", "two", "three"]);
        let result = filter.array_slice(input.clone(), 1, Some(3)).unwrap();
        assert_eq!(result, json!(["one", "two"]));
    }

    #[test]
    fn test_array_slice_no_end() {
        let filter = Filter::new();
        let input = json!(["zero", "one", "two", "three"]);
        let result = filter.array_slice(input.clone(), 2, None).unwrap();
        assert_eq!(result, json!(["two", "three"]));
    }

    #[test]
    fn test_array_slice_out_of_bounds() {
        let filter = Filter::new();
        let input = json!(["zero", "one", "two", "three"]);
        let result = filter.array_slice(input, 3, Some(5));
        assert!(matches!(result, Err(FilterError::SliceOutOfBounds)));
    }

    #[test]
    fn test_add_numbers() {
        let filter = Filter::new();
        let input = json!([1, 2, 3]);
        let result = filter.add(input).unwrap();
        assert_eq!(result, json!(6.0));
    }

    #[test]
    fn test_add_floats() {
        let filter = Filter::new();
        let input = json!([1.5, 2.5, 3.0]);
        let result = filter.add(input).unwrap();
        assert_eq!(result, json!(7.0));
    }

    #[test]
    fn test_add_strings() {
        let filter = Filter::new();
        let input = json!(["one", "two", "three"]);
        let result = filter.add(input).unwrap();
        assert_eq!(result, json!("onetwothree"));
    }

    #[test]
    fn test_add_invalid_types() {
        let filter = Filter::new();
        let input = json!([true, false]);
        let result = filter.add(input);
        assert!(matches!(result, Err(FilterError::InvalidType)));
    }

    #[test]
    fn test_length_array() {
        let filter = Filter::new();
        let input = json!([1, 2, 3, 4]);
        let result = filter.length(input).unwrap();
        assert_eq!(result, json!(4));
    }

    #[test]
    fn test_length_object() {
        let filter = Filter::new();
        let input = json!({"a": 1, "b": 2, "c": 3});
        let result = filter.length(input).unwrap();
        assert_eq!(result, json!(3));
    }

    #[test]
    fn test_length_string() {
        let filter = Filter::new();
        let input = json!("hello");
        let result = filter.length(input).unwrap();
        assert_eq!(result, json!(5));
    }

    #[test]
    fn test_length_invalid_type() {
        let filter = Filter::new();
        let input = json!(null);
        let result = filter.length(input);
        assert!(matches!(result, Err(FilterError::InvalidType)));
    }

    #[test]
    fn test_del_key() {
        let filter = Filter::new();
        let input = json!({"key": "value", "other": "data"});
        let target = FilterFn::KeyFilter("key".to_string());
        let result = filter.del(input, &target).unwrap();
        assert_eq!(result, json!({"other": "data"}));
    }

    #[test]
    fn test_del_nonexistent_key() {
        let filter = Filter::new();
        let input = json!({"key": "value"});
        let target = FilterFn::KeyFilter("nonexistent".to_string());
        let result = filter.del(input.clone(), &target).unwrap();
        // Key doesn't exist, but input remains unchanged
        assert_eq!(result, input);
    }

    #[test]
    fn test_del_array_index() {
        let filter = Filter::new();
        let input = json!(["zero", "one", "two"]);
        let target = FilterFn::ArrayIndex(1);
        let result = filter.del(input, &target).unwrap();
        assert_eq!(result, json!(["zero", "two"]));
    }

    #[test]
    fn test_del_array_index_out_of_bounds() {
        let filter = Filter::new();
        let input = json!(["zero", "one", "two"]);
        let target = FilterFn::ArrayIndex(5);
        let result = filter.del(input, &target);
        assert!(matches!(result, Err(FilterError::IndexOutOfBounds(_))));
    }

    #[test]
    fn test_del_invalid_target() {
        let filter = Filter::new();
        let input = json!({"key": "value"});
        let target = FilterFn::Add; // Invalid target for del
        let result = filter.del(input, &target);
        assert!(matches!(result, Err(FilterError::InvalidType)));
    }
}
