use md5;
use serde_json::Value;

mod notation;
mod sorter;

use notation::Notation;

pub fn signature(value: Value) -> String {
    format!("{:x}", md5::compute(value.notate()))
}

fn json_from(input: &str) -> Value {
    serde_json::from_str(input).expect("Invalid JSON")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_for_string() {
        let data: Value = json_from(r#""abc""#);
        let result: String = signature(data);
        assert_eq!(result, "c4449120506d97975c67be69719a78e2");
    }

    #[test]
    fn signature_for_number() {
        let data: Value = json_from(r#"1"#);
        let result: String = signature(data);
        assert_eq!(result, "594170053719896a11eb08ee513813d5");
    }

    #[test]
    fn signature_for_float() {
        let data: Value = json_from(r#"1.2"#);
        let result: String = signature(data);
        assert_eq!(result, "f1ab6592886cd4b1b66ed55e73d9ab81");
    }

    #[test]
    fn signature_for_array_of_integers() {
        let data: Value = json_from(r#"[1, 2, 3]"#);
        let result: String = signature(data);
        assert_eq!(result, "b07db153d855dc2a42a0b669e3f7e4b3");
    }

    #[test]
    fn signature_for_array_of_strings() {
        let data: Value = json_from(r#"["a", "b", "c"]"#);
        let result: String = signature(data);
        assert_eq!(result, "c732c2fd36a2573974fe22f20a24e4f9");
    }

    #[test]
    fn signature_for_array_mixed() {
        let data: Value = json_from(r#"[1, "a", 3]"#);
        let result: String = signature(data);
        assert_eq!(result, "cd1c43797d488d0f6c0d71537c64d30b")
    }

    #[test]
    fn signature_for_array_mixed_sorting() {
        let data: Value = json_from(r#"[3, null, 1, "1"]"#);
        let result: String = signature(data);
        assert_eq!(result, "518e7bb17674f6acbb296845862a152d")
    }

    #[test]
    fn signature_for_array_letter_casing() {
        let data: Value = json_from(r#"["a", "A", "b", "B"]"#);
        let result: String = signature(data);
        assert_eq!(result, "f6692ab4bc94b35e61ec15c2d1891734")
    }

    #[test]
    fn signature_for_nested_arrays() {
        let data: Value = json_from(r#"["a", 1, ["b", "2"]]"#);
        let result: String = signature(data);
        assert_eq!(result, "3aaa58da4841eaeb41d3726d2c6fd875")
    }

    #[test]
    fn signature_for_nested_arrays_different_order() {
        let data: Value = json_from(r#"[["b", "2"], "a", 1]"#);
        let result: String = signature(data);
        assert_eq!(result, "3aaa58da4841eaeb41d3726d2c6fd875")
    }

    #[test]
    fn signature_for_hash_data_structure() {
        let data: Value = json_from(r#"{"a": 1}"#);
        let result: String = signature(data);
        assert_eq!(result, "8cb44d69badda0f34b0bab6bb3e7fdbf")
    }

    #[test]
    fn signature_for_nested_hash_data_structure() {
        let data: Value = json_from(r#"{"a": {"c": null, "2": 2 }}"#);
        let result: String = signature(data);
        assert_eq!(result, "bff3538075e4007c7679a7ba0d0a5f30")
    }

    #[test]
    fn signature_for_null_value() {
        let data: Value = json_from(r#"null"#);
        let result: String = signature(data);
        assert_eq!(result, "b14a7b8059d9c055954c92674ce60032")
    }

    #[test]
    fn signature_for_true_boolean_value() {
        let data: Value = json_from(r#"true"#);
        let result: String = signature(data);
        assert_eq!(result, "6413cfeb7a89f7e0a8872f82b919c0d9")
    }

    #[test]
    fn notate_false_boolean_value() {
        let data: Value = json_from(r#"false"#);
        let result: String = signature(data);
        assert_eq!(result, "fa39253035cfe44c8638b8f5d7a3402e")
    }
}
