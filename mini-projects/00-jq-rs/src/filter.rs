use serde_json::Value;

pub trait Filter {
    fn filter(&self, value: &Value) -> bool;
}

pub struct add {
    key: String,
}
