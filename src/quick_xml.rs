//! Helper functions for quick xml crate.

use quick_xml::SeError;
use quick_xml::se::{Serializer, WriteResult};
use serde::Serialize;
use std::fmt::Write;

/// Serializes a value to xml format with indentation.
pub fn to_string_indent<T>(
    value: &T,
    indent_char: char,
    indent_size: usize,
) -> Result<String, SeError>
where
    T: ?Sized + Serialize,
{
    let mut buffer = String::new();
    to_writer_indent(&mut buffer, value, indent_char, indent_size)?;
    Ok(buffer)
}

/// Serializes a value to xml format with indentation and writes it to the provided writer.
pub fn to_writer_indent<W, T>(
    mut writer: W,
    value: &T,
    indent_char: char,
    indent_size: usize,
) -> Result<WriteResult, SeError>
where
    W: Write,
    T: ?Sized + Serialize,
{
    let mut serializer = Serializer::new(&mut writer);
    serializer.indent(indent_char, indent_size);
    value.serialize(serializer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct TestStruct {
        foo: String,
        bar: i32,
    }

    #[test]
    fn test_to_string_indent() {
        let value = TestStruct {
            foo: "xyz".to_string(),
            bar: 7,
        };
        let xml = to_string_indent(&value, ' ', 4).unwrap();
        assert_eq!(
            xml,
            r#"<TestStruct>
    <foo>xyz</foo>
    <bar>7</bar>
</TestStruct>"#
        );
    }
}
