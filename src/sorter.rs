use serde_json::Value;

fn sorter(mut data: Value) -> Value {
    sort_value(&mut data);
    data
}

fn sort_value(value: &mut Value) {
    match value {
        Value::Array(arr) => {
            for elem in arr.iter_mut() {
                sort_value(elem);
            }
            arr.sort_by(|a: &Value, b: &Value| {
                string_to_ascii(normalise_value(a)).cmp(&string_to_ascii(normalise_value(b)))
            });
        }
        _ => {}
    }
}

fn normalise_value(value: &Value) -> String {
    match &value {
        Value::String(_s) => value.as_str().unwrap().to_string(),
        Value::Array(_a) => Value::to_string(&value),
        _ => return format!("{:?}", Value::to_string(&value)),
    }
}

fn string_to_ascii(input: String) -> Vec<u8> {
    input.chars().map(|c| c as u8).collect()
}

fn json_from(input: &str) -> Value {
    serde_json::from_str(input).expect("Invalid JSON")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorter_of_data() {
        let data: Value = json_from(
            r#"[7, ["b", 2], [4, "b"], [1, 2], ["c", "b"], "c", 3, [2, "a"], [3, 2], ["a", 1]]"#,
        );
        let result: Value = sorter(data);
        let expected: Value = json_from(
            r#"[3, 7, ["b", "c"], [1, "a"], [1, 2], [2, "a"], [2, "b"], [2, 3], [4, "b"], "c"]"#,
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn sort_short_array_of_data() {
        let data: Value = json_from(r#"[["c", "b"], ["a", 1]]"#);
        let result: Value = sorter(data);
        let expected: Value = json_from(r#"[["b", "c"], [1, "a"]]"#);
        assert_eq!(result, expected);
    }

    #[test]
    fn sort_array_of_simple_values() {
        let data: Value = json_from(r#"["a", 1]"#);
        let result: Value = sorter(data);
        let expected: Value = json_from(r#"[1, "a"]"#);
        assert_eq!(result, expected);
    }

    #[test]
    fn sort_array_of_simple_numbers() {
        let data: Value = json_from(r#"[2, 1]"#);
        let result: Value = sorter(data);
        let expected: Value = json_from(r#"[1, 2]"#);
        assert_eq!(result, expected);
    }
    #[test]

    fn sort_array_of_simple_strings() {
        let data: Value = json_from(r#"["b", "a"]"#);
        let result: Value = sorter(data);
        let expected: Value = json_from(r#"["a", "b"]"#);
        assert_eq!(result, expected);
    }

    #[test]
    fn sort_array_of_simple_items() {
        let data: Value = json_from(r#"[[1], "c", ["b"]]"#);
        let result: Value = sorter(data);
        let expected: Value = json_from(r#"[["b"], [1], "c"]"#);
        assert_eq!(result, expected);
    }

    #[test]
    fn sort_floating_point() {
        let data: Value = json_from(r#"[7.1, 3.2, 1.6]"#);
        let result: Value = sorter(data);
        let expected: Value = json_from(r#"[1.6, 3.2, 7.1]"#);
        assert_eq!(result, expected);
    }

    // @todo add a test for a non-ascii character
}
