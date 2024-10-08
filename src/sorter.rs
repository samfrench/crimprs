use serde_json::Value;

pub fn sorter(mut data: Value) -> Value {
    sort_value(&mut data);
    data
}

fn sort_value(value: &mut Value) {
    match value {
        Value::Array(arr) => {
            arr.sort_by(|a: &Value, b: &Value| {
                string_to_ascii(normalise_value(a)).cmp(&string_to_ascii(normalise_value(b)))
            });
        }
        Value::Object(obj) => {
            let mut data: Vec<(String, Value)> = Vec::new();

            for (key, value) in obj.iter_mut() {
                data.push((key.clone(), value.clone()));
            }

            *value = Value::Array(
                data.into_iter()
                    .map(|(k, v)| Value::Array(vec![Value::String(k), v]))
                    .collect(),
            );
        }
        _ => {}
    }
}

fn normalise_value(value: &Value) -> String {
    match &value {
        Value::String(_s) => value.as_str().unwrap().to_string(),
        Value::Array(_a) => Value::to_string(&value),
        Value::Null => "".to_string(),
        _ => Value::to_string(&value),
    }
}

fn string_to_ascii(input: String) -> Vec<u8> {
    input.chars().map(|c| c as u8).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::json_from;

    #[test]
    fn sorter_of_data() {
        let data: Value = json_from(
            r#"[7, ["b", 2], [4, "b"], [1, 2], ["c", "b"], "c", 3, [2, "a"], [3, 2], ["a", 1]]"#,
        );
        let result: Value = sorter(data);
        // If this did deep sorting this would be the expected output.
        // This is now done iteratively in the notation.
        // let expected: Value = json_from(
        //     r#"[3, 7, ["b", "c"], [1, "a"], [1, 2], [2, "a"], [2, "b"], [2, 3], [4, "b"], "c"]"#,
        // );
        let expected: Value = json_from(
            r#"[3, 7, ["a", 1], ["b", 2], ["c", "b"], [1, 2], [2, "a"], [3, 2], [4, "b"], "c"]"#,
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn sort_short_array_of_data() {
        let data: Value = json_from(r#"[["c", "b"], ["a", 1]]"#);
        let result: Value = sorter(data);
        // If this did deep sorting this would be the expected output.
        // This is now done iteratively in the notation.
        // let expected: Value = json_from(r#"[["b", "c"], [1, "a"]]"#);
        let expected: Value = json_from(r#"[["a", 1], ["c", "b"]]"#);
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

    #[test]
    fn sort_array_of_same_values_different_types() {
        let data: Value = json_from(r#"[1, "1", 2, "2"]"#);
        let result: Value = sorter(data);
        let expected: Value = json_from(r#"[1, "1", 2, "2"]"#);
        assert_eq!(result, expected);
    }

    #[test]
    fn sort_array_of_simple_top_level_values() {
        let data: Value = json_from(r#"[3, null, 1, "1"]"#);
        let result: Value = sorter(data);
        let expected: Value = json_from(r#"[null, 1, "1", 3]"#);
        assert_eq!(result, expected);
    }

    #[test]
    fn sort_hash() {
        let data: Value = json_from(r#"{"a": 1}"#);
        let result: Value = sorter(data);
        let expected: Value = json_from(r#"[["a", 1]]"#);
        assert_eq!(result, expected);
    }

    #[test]
    fn sort_nested_hash_data_structure() {
        // If this did deep sorting this would be the expected output.
        // This is now done iteratively in the notation.
        // let expected: Value = json_from(r#"[["a", [["2", 2], ["c", null]]]]"#);
        let data: Value = json_from(r#"{"a": {"c": null, "2": 2 }}"#);
        let result: Value = sorter(data);
        let expected: Value = json_from(r#"[["a", {"2": 2, "c": null}]]"#);
        assert_eq!(result, expected);
    }

    // @todo add a test for a non-ascii character
}
