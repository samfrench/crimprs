use core::fmt;
// use std::{any::Any, cmp::Ordering};
use std::cmp::Ordering;

// #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Value {
    Null(),
    Number(i32),
    Array(Vec<Value>),
    String(String),
    Alphanumeric(Alphanumeric),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if matches!(self, Value::Array(_)) && matches!(other, Value::Array(_)) {
            return Ordering::Equal;
        }

        self.to_string().cmp(&other.to_string())
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //   write!(f, "{:?}", self)
        match self {
            Value::Null() => write!(f, ""),
            Value::Number(n) => write!(f, "{}", n.to_string()),
            // Value::Alphanumeric(a) => write!(f, "{}", a.to_string()),
            Value::Alphanumeric(_a) => todo!(),
            Value::String(s) => write!(f, "{}", s.to_string()),
            Value::Array(a) => write!(
                f,
                "{}",
                a.iter()
                    .fold(String::new(), |acc, arg| acc + &arg.to_string())
            ),
            // Value::Array(a) => write!(f, "{}", Vec::from_iter(a.iter().map(|i| i.to_string())).join(", ")),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Alphanumeric {
    pub(crate) value: String,
    pub(crate) raw: Raw,
    pub(crate) is_number: bool,
    pub(crate) is_string: bool,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum Raw {
    Number(i32),
    String(String),
}
