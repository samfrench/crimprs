use crate::value::Value;

trait Notation {
    fn notate(&self) -> String;
}

impl Notation for String {
    fn notate(&self) -> String {
        format!("{}S", &self)
    }
}

impl Notation for i32 {
    fn notate(&self) -> String {
        format!("{}N", &self)
    }
}

impl Notation for i64 {
    fn notate(&self) -> String {
        format!("{}N", &self)
    }
}

impl Notation for f32 {
    fn notate(&self) -> String {
        format!("{}N", &self)
    }
}

impl Notation for [i32] {
    fn notate(&self) -> String {
        format!("{}A", &self.iter().map(|x| x.notate()).collect::<String>())
    }
}

impl Notation for [&str] {
    fn notate(&self) -> String {
        format!(
            "{}A",
            &self
                .iter()
                .map(|x| x.to_string().notate())
                .collect::<String>()
        )
    }
}

impl Notation for Value {
    fn notate(&self) -> String {
        match self {
            Value::Number(n) => n.notate(),
            Value::Null() => "_".to_string(),
            // Value::Bool(_) => todo!(),
            Value::String(s) => s.notate(),
            Value::Alphanumeric(a) => {
                if a.is_number {
                    format!("{}N", a.value)
                } else {
                    format!("{}S", a.value)
                }
            }
            Value::Array(a) => a.notate(),
        }
    }
}

impl Notation for [Value] {
    fn notate(&self) -> String {
        let data = &self.to_vec();

        format!("{}A", &data.iter().map(|x| x.notate()).collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notate_string() {
        let result: String = "abc".to_string().notate();
        assert_eq!(result, "abcS");
    }

    #[test]
    fn notate_integer() {
        let result: String = 1_i32.notate();
        assert_eq!(result, "1N");
    }

    #[test]
    fn notate_float() {
        let result: String = 1.2_f32.notate();
        assert_eq!(result, "1.2N");
    }

    #[test]
    fn notate_array_of_integers() {
        let result: String = [1, 2, 3].notate();
        assert_eq!(result, "1N2N3NA");
    }

    #[test]
    fn notate_array_of_strings() {
        let result: String = ["a", "b", "c"].notate();
        assert_eq!(result, "aSbScSA");
    }

    #[test]
    fn notate_array_mixed() {
        let data = [
            Value::Number(1),
            Value::String("a".to_string()),
            Value::Number(3),
        ];
        let result: String = data.notate();
        assert_eq!(result, "1N3NaSA")
    }

    #[test]
    fn notate_array_mixed_sorting() {
        let data = [
            Value::Number(3),
            Value::Null(),
            Value::Number(1),
            Value::String("1".to_string()),
        ];
        let result: String = data.notate();
        assert_eq!(result, "_1N1S3NA")
    }

    #[test]
    fn notate_array_letter_casing() {
        let data = [
            Value::String("a".to_string()),
            Value::String("A".to_string()),
            Value::String("b".to_string()),
            Value::String("B".to_string()),
        ];
        let result: String = data.notate();
        assert_eq!(result, "ASBSaSbSA")
    }

    #[test]
    fn notate_nested_arrays() {
        let mut data = [
            Value::String("a".to_string()),
            Value::Number(1),
            Value::Array(vec![
                Value::String("b".to_string()),
                Value::String("2".to_string()),
            ]),
        ];
        let result: String = data.notate();
        assert_eq!(result, "1N2SbSAaSA")
    }

    // desc: verify nested Arrays
    // json: ["a", 1, ["b", "2"]]
    // string: 1N2SbSAaSA
    // signature: 3aaa58da4841eaeb41d3726d2c6fd875

    // desc: verify nested Arrays
    // json: [["b", "2"], "a", 1]
    // string: 1N2SbSAaSA
    // signature: 3aaa58da4841eaeb41d3726d2c6fd875

    // desc: verify hash like data structures
    // json: {"a": 1}
    // string: 1NaSAH
    // signature: 8cb44d69badda0f34b0bab6bb3e7fdbf

    // desc: verify nested hash
    // json: {"a": {"c": null, "2": 2 }}
    // string: aS2S2NA_cSAHAH
    // signature: bff3538075e4007c7679a7ba0d0a5f30

    // desc: verify null values
    // json: null
    // string: _
    // signature: b14a7b8059d9c055954c92674ce60032

    // desc: verify true boolean values
    // json: true
    // string: trueB
    // signature: 6413cfeb7a89f7e0a8872f82b919c0d9

    // desc: verify false boolean values
    // json: false
    // string: falseB
    // signature: fa39253035cfe44c8638b8f5d7a3402e

    // #[test]
    // fn notate_array_sorting() {
    //   let three_n = Alphanumeric{
    //     raw: Raw::Number(3),
    //     value: 3.to_string(),
    //     is_number: true,
    //     is_string: false
    //   };

    //   let one_n = Alphanumeric{
    //     raw: Raw::Number(1),
    //     value: 1.to_string(),
    //     is_number: true,
    //     is_string: false
    //   };

    //   let one_s = Alphanumeric{
    //     raw: Raw::String("1".to_string()),
    //     value: "1".to_string(),
    //     is_number: false,
    //     is_string: true
    //   };

    //   let mut data = [Value::Alphanumeric(three_n), Value::Null(), Value::Alphanumeric(one_n), Value::Alphanumeric(one_s)];
    //   data.sort();
    //   let result: String = data.notate();
    //   assert_eq!(result, "_1N1S3NA");
    // }
}
