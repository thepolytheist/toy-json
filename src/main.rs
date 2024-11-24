use parser::Parser;
use tokenizer::tokenize;
use writer::JsonWriter;

mod tokenizer;
mod parser;
mod writer;

fn main() {
    let json_string = "{\n  \"object\": {\n    \"array\": [\n      \"string\",\n      {},\n      [\"nested array string\"]\n    ]\n  }\n}";
    println!("Initial JSON string:\n{}\n", json_string);
    println!("Token stream from string:\n{:?}\n", tokenize(json_string).unwrap());
    let json_value = Parser::parse_string(json_string.into()).unwrap();
    println!("Internal JsonValue representation:\n{:?}\n", json_value);
    println!("Parsed token stream written back to JSON string:\n{}", JsonWriter::write(json_value));
}

#[test]
fn parse_object_with_one_string_member() {
    let json_string = "{\"user\": \"thepolytheist\"}";
    let object = Parser::parse_string(json_string.into()).unwrap();
    assert_eq!(json_string, JsonWriter::write(object));
}

#[test]
fn strip_whitespace() {
    let json_string_with_whitespace = "     {  \"whitespace\"    : \"true\"  }";
    let json_string = "{\"whitespace\": \"true\"}";
    let object = Parser::parse_string(json_string_with_whitespace.into()).unwrap();
    assert_eq!(json_string, JsonWriter::write(object));
}