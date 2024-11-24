# toy-json
## Summary
This project only exists to play around with Rust enums and pattern matching. This is by no means intended to be used for any actual JSON validation (see any `FIXME`s in the comments), nor is it intended to be an example of good Rust, good lexing, etc. As of right now, parsing numbers, bools, `null`, and strings with escape characters all do not work, but structure and scope are otherwise correct.

## Example
From `main.rs`:
```rust
fn main() {
    let json_string = "{\n  \"object\": {\n    \"array\": [\n      \"string\",\n      {},\n      [\"nested array string\"]\n    ]\n  }\n}";
    println!("Initial JSON string:\n{}\n", json_string);
    println!("Token stream from string:\n{:?}\n", tokenize(json_string).unwrap());
    let json_value = Parser::parse_string(json_string.into()).unwrap();
    println!("Internal JsonValue representation:\n{:?}\n", json_value);
    println!("Parsed token stream written back to JSON string:\n{}", JsonWriter::write(json_value));
}
```

Output:
```
Initial JSON string:
{
  "object": {
    "array": [
      "string",
      {},
      ["nested array string"]
    ]
  }
}

Token stream from string:
[LeftBrace, Identifier("object"), Colon, LeftBrace, Identifier("array"), Colon, LeftBracket, Identifier("string"), Comma, LeftBrace, RightBrace, Comma, LeftBracket, Identifier("nested array string"), RightBracket, RightBracket, RightBrace, RightBrace]

Internal JsonValue representation:
Object { members: {"object": Object { members: {"array": Array([String("string"), Object { members: {} }, Array([String("nested array string")])])} }} }

Parsed token stream written back to JSON string:
{"object": {"array": ["string", {}, ["nested array string"]]}}
```

## Reference
I made good use of [Geoffrey Copin's SQLite parser blog post](https://blog.sylver.dev/build-your-own-sqlite-part-3-sql-parsing-101) in determining the pattern I wanted to follow for my own parsing. 

I also, of course, followed the [JSON spec](https://www.json.org/json-en.html) religiously.

## Issues and Contribution
I have no plans to maintain issues nor approve PRs, but you're welcome to attempt to persuade me otherwise. If you have any kind suggestions for improvements, I'm all ears.