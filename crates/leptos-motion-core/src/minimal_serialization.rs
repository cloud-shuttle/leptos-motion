//! Minimal serialization implementation for Phase 4 optimization
//!
//! This module provides lightweight serialization alternatives to reduce
//! bundle size by replacing heavy serde usage with custom minimal implementations.

use std::fmt;

/// Minimal serialization trait for lightweight serialization
pub trait MinimalSerialize {
    /// Serialize to a minimal string representation
    fn to_minimal_string(&self) -> String;
}

/// Minimal deserialization trait for lightweight deserialization
pub trait MinimalDeserialize: Sized {
    /// Deserialize from a minimal string representation
    fn from_minimal_string(s: &str) -> Result<Self, MinimalSerializationError>;
}

/// Error type for minimal serialization
#[derive(Debug, Clone, PartialEq)]
pub enum MinimalSerializationError {
    /// Invalid format
    InvalidFormat(String),
    /// Missing field
    MissingField(String),
    /// Type conversion error
    TypeConversion(String),
}

impl fmt::Display for MinimalSerializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MinimalSerializationError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            MinimalSerializationError::MissingField(field) => write!(f, "Missing field: {}", field),
            MinimalSerializationError::TypeConversion(msg) => {
                write!(f, "Type conversion error: {}", msg)
            }
        }
    }
}

impl std::error::Error for MinimalSerializationError {}

/// Minimal JSON-like serialization for simple types
pub struct MinimalJsonSerializer;

impl MinimalJsonSerializer {
    /// Serialize a simple key-value pair
    pub fn serialize_kv(key: &str, value: &str) -> String {
        format!("\"{}\":\"{}\"", key, value)
    }

    /// Serialize a simple key-number pair
    pub fn serialize_kv_number(key: &str, value: f64) -> String {
        format!("\"{}\":{}", key, value)
    }

    /// Serialize a simple key-boolean pair
    pub fn serialize_kv_bool(key: &str, value: bool) -> String {
        format!("\"{}\":{}", key, if value { "true" } else { "false" })
    }

    /// Create a simple object
    pub fn create_object(fields: Vec<String>) -> String {
        format!("{{{}}}", fields.join(","))
    }

    /// Create a simple array
    pub fn create_array(items: Vec<String>) -> String {
        format!("[{}]", items.join(","))
    }

    /// Parse a simple key-value pair
    pub fn parse_kv(s: &str) -> Result<(String, String), MinimalSerializationError> {
        if let Some(colon_pos) = s.find(':') {
            let key_part = &s[..colon_pos];
            let value_part = &s[colon_pos + 1..];

            // Remove quotes from key
            let key = if key_part.starts_with('"') && key_part.ends_with('"') {
                key_part[1..key_part.len() - 1].to_string()
            } else {
                return Err(MinimalSerializationError::InvalidFormat(
                    "Key must be quoted".to_string(),
                ));
            };

            // Remove quotes from value if present
            let value = if value_part.starts_with('"') && value_part.ends_with('"') {
                value_part[1..value_part.len() - 1].to_string()
            } else {
                value_part.to_string()
            };

            Ok((key, value))
        } else {
            Err(MinimalSerializationError::InvalidFormat(
                "Missing colon".to_string(),
            ))
        }
    }

    /// Parse a simple object
    pub fn parse_object(s: &str) -> Result<Vec<(String, String)>, MinimalSerializationError> {
        let trimmed = s.trim();
        if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
            return Err(MinimalSerializationError::InvalidFormat(
                "Not an object".to_string(),
            ));
        }

        let content = &trimmed[1..trimmed.len() - 1];
        if content.trim().is_empty() {
            return Ok(vec![]);
        }

        let mut result = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut brace_count = 0;

        for ch in content.chars() {
            match ch {
                '"' => in_quotes = !in_quotes,
                '{' | '[' => {
                    if !in_quotes {
                        brace_count += 1;
                    }
                }
                '}' | ']' => {
                    if !in_quotes {
                        brace_count -= 1;
                    }
                }
                ',' => {
                    if !in_quotes && brace_count == 0 {
                        result.push(Self::parse_kv(current.trim())?);
                        current.clear();
                        continue;
                    }
                }
                _ => {}
            }
            current.push(ch);
        }

        if !current.trim().is_empty() {
            result.push(Self::parse_kv(current.trim())?);
        }

        Ok(result)
    }
}

/// Minimal binary serialization for compact representation
pub struct MinimalBinarySerializer;

impl MinimalBinarySerializer {
    /// Serialize a u32 to 4 bytes
    pub fn serialize_u32(value: u32) -> [u8; 4] {
        value.to_le_bytes()
    }

    /// Deserialize a u32 from 4 bytes
    pub fn deserialize_u32(bytes: [u8; 4]) -> u32 {
        u32::from_le_bytes(bytes)
    }

    /// Serialize a f64 to 8 bytes
    pub fn serialize_f64(value: f64) -> [u8; 8] {
        value.to_le_bytes()
    }

    /// Deserialize a f64 from 8 bytes
    pub fn deserialize_f64(bytes: [u8; 8]) -> f64 {
        f64::from_le_bytes(bytes)
    }

    /// Serialize a boolean to 1 byte
    pub fn serialize_bool(value: bool) -> u8 {
        if value { 1 } else { 0 }
    }

    /// Deserialize a boolean from 1 byte
    pub fn deserialize_bool(byte: u8) -> Result<bool, MinimalSerializationError> {
        match byte {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(MinimalSerializationError::TypeConversion(
                "Invalid boolean value".to_string(),
            )),
        }
    }
}

/// Compact string serialization for small strings
pub struct CompactStringSerializer;

impl CompactStringSerializer {
    /// Serialize a string with length prefix
    pub fn serialize_string(s: &str) -> Vec<u8> {
        let bytes = s.as_bytes();
        let mut result = Vec::with_capacity(4 + bytes.len());

        // Add length as u32 (4 bytes)
        result.extend_from_slice(&(bytes.len() as u32).to_le_bytes());

        // Add string bytes
        result.extend_from_slice(bytes);

        result
    }

    /// Deserialize a string with length prefix
    pub fn deserialize_string(data: &[u8]) -> Result<String, MinimalSerializationError> {
        if data.len() < 4 {
            return Err(MinimalSerializationError::InvalidFormat(
                "Data too short".to_string(),
            ));
        }

        let length = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;

        if data.len() < 4 + length {
            return Err(MinimalSerializationError::InvalidFormat(
                "Incomplete string data".to_string(),
            ));
        }

        let string_bytes = &data[4..4 + length];
        String::from_utf8(string_bytes.to_vec())
            .map_err(|e| MinimalSerializationError::TypeConversion(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal_json_serializer_basic() {
        let kv = MinimalJsonSerializer::serialize_kv("name", "value");
        assert_eq!(kv, "\"name\":\"value\"");

        let kv_num = MinimalJsonSerializer::serialize_kv_number("count", 42.5);
        assert_eq!(kv_num, "\"count\":42.5");

        let kv_bool = MinimalJsonSerializer::serialize_kv_bool("enabled", true);
        assert_eq!(kv_bool, "\"enabled\":true");
    }

    #[test]
    fn test_minimal_json_serializer_objects() {
        let fields = vec![
            MinimalJsonSerializer::serialize_kv("name", "test"),
            MinimalJsonSerializer::serialize_kv_number("count", 10.0),
        ];
        let obj = MinimalJsonSerializer::create_object(fields);
        assert_eq!(obj, "{\"name\":\"test\",\"count\":10}");

        let items = vec!["\"item1\"".to_string(), "\"item2\"".to_string()];
        let arr = MinimalJsonSerializer::create_array(items);
        assert_eq!(arr, "[\"item1\",\"item2\"]");
    }

    #[test]
    fn test_minimal_json_serializer_parsing() {
        let (key, value) = MinimalJsonSerializer::parse_kv("\"name\":\"value\"").unwrap();
        assert_eq!(key, "name");
        assert_eq!(value, "value");

        let (key, value) = MinimalJsonSerializer::parse_kv("\"count\":42").unwrap();
        assert_eq!(key, "count");
        assert_eq!(value, "42");
    }

    #[test]
    fn test_minimal_json_serializer_object_parsing() {
        let obj_str = "{\"name\":\"test\",\"count\":10}";
        let fields = MinimalJsonSerializer::parse_object(obj_str).unwrap();
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0], ("name".to_string(), "test".to_string()));
        assert_eq!(fields[1], ("count".to_string(), "10".to_string()));

        let empty_obj = "{}";
        let empty_fields = MinimalJsonSerializer::parse_object(empty_obj).unwrap();
        assert_eq!(empty_fields.len(), 0);
    }

    #[test]
    fn test_minimal_binary_serializer() {
        let original_u32 = 12345u32;
        let bytes = MinimalBinarySerializer::serialize_u32(original_u32);
        let deserialized = MinimalBinarySerializer::deserialize_u32(bytes);
        assert_eq!(original_u32, deserialized);

        let original_f64 = 3.14159f64;
        let bytes = MinimalBinarySerializer::serialize_f64(original_f64);
        let deserialized = MinimalBinarySerializer::deserialize_f64(bytes);
        assert_eq!(original_f64, deserialized);

        let original_bool = true;
        let byte = MinimalBinarySerializer::serialize_bool(original_bool);
        let deserialized = MinimalBinarySerializer::deserialize_bool(byte).unwrap();
        assert_eq!(original_bool, deserialized);
    }

    #[test]
    fn test_compact_string_serializer() {
        let original = "Hello, World!";
        let serialized = CompactStringSerializer::serialize_string(original);
        let deserialized = CompactStringSerializer::deserialize_string(&serialized).unwrap();
        assert_eq!(original, deserialized);

        let empty = "";
        let serialized_empty = CompactStringSerializer::serialize_string(empty);
        let deserialized_empty =
            CompactStringSerializer::deserialize_string(&serialized_empty).unwrap();
        assert_eq!(empty, deserialized_empty);
    }

    #[test]
    fn test_compact_string_serializer_errors() {
        // Test with data too short
        let result = CompactStringSerializer::deserialize_string(&[1, 2]);
        assert!(result.is_err());

        // Test with incomplete data
        let result = CompactStringSerializer::deserialize_string(&[5, 0, 0, 0, 1, 2]);
        assert!(result.is_err());
    }
}
