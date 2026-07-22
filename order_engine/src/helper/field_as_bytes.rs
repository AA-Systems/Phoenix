use std::collections::HashMap;

use redis::Value;

pub fn field_as_bytes(fields: &HashMap<String, Value>, key: &str) -> Option<Vec<u8>> {
    match fields.get(key)? {
        Value::BulkString(bytes) => Some(bytes.clone()),
        Value::SimpleString(text) => Some(text.as_bytes().to_vec()),
        _ => None,
    }
}
