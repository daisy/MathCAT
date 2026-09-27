//! YAML scalar quoting helpers shared by the pretty printer and Rules packager.

/// Words that YAML may parse as booleans or null, so string values need quotes.
pub(crate) const RESERVED_WORDS: &[&str] = &[
    "yes", "Yes", "YES", "no", "No", "NO", "True", "TRUE", "true", "False", "FALSE",
    "false", "on", "On", "ON", "off", "Off", "OFF", "null", "Null", "NULL", "~",
];

/// Returns the escape sequence for a byte in a double-quoted YAML string.
/// Returns `None` when the byte can be written without escaping.
pub(crate) fn escaped_byte(byte: u8) -> Option<&'static str> {
    Some(match byte {
        b'"' => "\\\"",
        b'\\' => "\\\\",
        b'\x00' => "\\u0000",
        b'\x01' => "\\u0001",
        b'\x02' => "\\u0002",
        b'\x03' => "\\u0003",
        b'\x04' => "\\u0004",
        b'\x05' => "\\u0005",
        b'\x06' => "\\u0006",
        b'\x07' => "\\u0007",
        b'\x08' => "\\b",
        b'\t' => "\\t",
        b'\n' => "\\n",
        b'\x0b' => "\\u000b",
        b'\x0c' => "\\f",
        b'\r' => "\\r",
        b'\x0e' => "\\u000e",
        b'\x0f' => "\\u000f",
        b'\x10' => "\\u0010",
        b'\x11' => "\\u0011",
        b'\x12' => "\\u0012",
        b'\x13' => "\\u0013",
        b'\x14' => "\\u0014",
        b'\x15' => "\\u0015",
        b'\x16' => "\\u0016",
        b'\x17' => "\\u0017",
        b'\x18' => "\\u0018",
        b'\x19' => "\\u0019",
        b'\x1a' => "\\u001a",
        b'\x1b' => "\\u001b",
        b'\x1c' => "\\u001c",
        b'\x1d' => "\\u001d",
        b'\x1e' => "\\u001e",
        b'\x1f' => "\\u001f",
        b'\x7f' => "\\u007f",
        _ => return None,
    })
}
