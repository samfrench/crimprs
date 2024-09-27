use serde_json::Value;

use crate::sorter::sorter;

pub trait Notation {
    fn notate(&self) -> String;
}

impl Notation for String {
    fn notate(&self) -> String {
        format!("{}S", &self)
    }
}

impl Notation for serde_json::Number {
    fn notate(&self) -> String {
        format!("{}N", &self)
    }
}

impl Notation for bool {
    fn notate(&self) -> String {
        format!("{}B", &self)
    }
}

impl Notation for Value {
    fn notate(&self) -> String {
        match self {
            Value::Number(n) => n.notate(),
            Value::Null => "_".to_string(),
            Value::Bool(b) => b.notate(),
            Value::String(s) => s.notate(),
            Value::Array(_a) => {
                let data = sorter(self.clone());

                format!(
                    "{}A",
                    &data
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|x| x.notate())
                        .collect::<String>()
                )
            }
            Value::Object(_o) => {
                let data = sorter(self.clone());

                format!(
                    "{}H",
                    &data
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|x| x.notate())
                        .collect::<String>()
                )
            }
        }
    }
}

fn json_from(input: &str) -> Value {
    serde_json::from_str(input).expect("Invalid JSON")
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    #[test]
    fn notate_string() {
        let data: Value = json_from(r#""abc""#);
        let result: String = data.notate();
        assert_eq!(result, "abcS");
    }

    #[test]
    fn notate_number() {
        let data: Value = json_from(r#"1"#);
        let result: String = data.notate();
        assert_eq!(result, "1N");
    }

    #[test]
    fn notate_float() {
        let data: Value = json_from(r#"1.2"#);
        let result: String = data.notate();
        assert_eq!(result, "1.2N");
    }

    #[test]
    fn notate_array_of_integers() {
        let data: Value = json_from(r#"[1, 2, 3]"#);
        let result: String = data.notate();
        assert_eq!(result, "1N2N3NA");
    }

    #[test]
    fn notate_array_of_strings() {
        let data: Value = json_from(r#"["a", "b", "c"]"#);
        let result: String = data.notate();
        assert_eq!(result, "aSbScSA");
    }

    #[test]
    fn notate_array_mixed() {
        let data: Value = json_from(r#"[1, "a", 3]"#);
        let result: String = data.notate();
        assert_eq!(result, "1N3NaSA")
    }

    #[test]
    fn notate_array_mixed_sorting() {
        let data: Value = json_from(r#"[3, null, 1, "1"]"#);
        let result: String = data.notate();
        assert_eq!(result, "_1N1S3NA")
    }

    #[test]
    fn notate_array_letter_casing() {
        let data: Value = json_from(r#"["a", "A", "b", "B"]"#);
        let result: String = data.notate();
        assert_eq!(result, "ASBSaSbSA")
    }

    #[test]
    fn notate_nested_arrays() {
        let data: Value = json_from(r#"["a", 1, ["b", "2"]]"#);
        let result: String = data.notate();
        assert_eq!(result, "1N2SbSAaSA")
    }

    #[test]
    fn notate_nested_arrays_different_order() {
        let data: Value = json_from(r#"[["b", "2"], "a", 1]"#);
        let result: String = data.notate();
        assert_eq!(result, "1N2SbSAaSA")
    }

    #[test]
    fn notate_hash_data_structure() {
        let data: Value = json_from(r#"{"a": 1}"#);
        let result: String = data.notate();
        assert_eq!(result, "1NaSAH")
    }

    #[test]
    fn notate_nested_hash_data_structure() {
        let data: Value = json_from(r#"{"a": {"c": null, "2": 2 }}"#);
        let result: String = data.notate();
        assert_eq!(result, "aS2S2NA_cSAHAH")
    }

    #[test]
    fn notate_null_value() {
        let data: Value = json_from(r#"null"#);
        let result: String = data.notate();
        assert_eq!(result, "_")
    }

    #[test]
    fn notate_true_boolean_value() {
        let data: Value = json_from(r#"true"#);
        let result: String = data.notate();
        assert_eq!(result, "trueB")
    }

    #[test]
    fn notate_false_boolean_value() {
        let data: Value = json_from(r#"false"#);
        let result: String = data.notate();
        assert_eq!(result, "falseB")
    }
}
