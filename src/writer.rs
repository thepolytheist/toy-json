use crate::parser::json_value::JsonValue;

/// Writes a [JsonValue] to [String].
pub struct JsonWriter;

impl JsonWriter {
    pub fn write(obj: JsonValue) -> String {
        // TODO: The idea behind the JsonWriter was that maybe this struct would handle pretty printing.
        format!("{}", obj)
    }
}