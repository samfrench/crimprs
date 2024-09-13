use serde_json::Value;

mod notation;
mod value;

pub fn signature(value: Value) -> String {
    serde_json::to_string(&value).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_for_value() {
        let result = signature(serde_json::json!({"a": 1}));
        assert_eq!(result, "{\"a\":1}");
    }
}
