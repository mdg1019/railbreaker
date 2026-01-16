use serde_json::{Map, Value};

fn to_camel_case_key(key: &str) -> String {
    let mut out = String::with_capacity(key.len());
    let mut chars = key.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '_' {
            if let Some(next) = chars.next() {
                out.push(next.to_ascii_uppercase());
            }
        } else {
            out.push(ch);
        }
    }

    out
}

pub fn to_camel_case_value(value: Value) -> Value {
    match value {
        Value::Array(items) => {
            Value::Array(items.into_iter().map(to_camel_case_value).collect())
        }
        Value::Object(map) => {
            let mut next = Map::new();
            for (key, value) in map {
                next.insert(to_camel_case_key(&key), to_camel_case_value(value));
            }
            Value::Object(next)
        }
        other => other,
    }
}
