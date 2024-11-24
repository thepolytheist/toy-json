use std::{collections::HashMap, fmt::Display};

/// Enum representation of the JSON specification components.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JsonValue {
    Object { members: HashMap<String, JsonValue> },
    Array(Vec<JsonValue>),
    String(String),
    Number(usize),
    Boolean(bool),
    Null
}

impl Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            JsonValue::Object { members } => format!("{{{}}}", members.into_iter()
                                                .map(|(k, v)| format!("\"{k}\": {v}"))
                                                .collect::<Vec<String>>()
                                                .join(", ")),
            JsonValue::Array(values) => format!("[{}]", values.into_iter()
                                            .map(|v| v.to_string())
                                            .collect::<Vec<String>>()
                                            .join(", ")),
            JsonValue::String(string) => format!("\"{}\"", string.to_owned()),
            JsonValue::Number(number) => number.to_string(),
            JsonValue::Boolean(bool) => bool.to_string(),
            JsonValue::Null => "null".into()
        })
    }
}
