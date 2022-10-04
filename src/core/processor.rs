
use std::io::{BufRead, Write};

use crate::core::error::FilterError;

/// 1. Read all bytes as UTF-8 String
/// 2. Run f with String
/// 3. Write the result of 2
pub fn run_all_input_as_utf8<R, W, F>(mut reader: R, mut writer: W, f: F) -> Result<(), FilterError>
where
    F: FnOnce(&str) -> Result<String, FilterError>,
    R: BufRead,
    W: Write,
{
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    let out = f(&buffer)?;

    writer.write_all(out.as_bytes())?;
    Ok(())
}

pub fn do_nothing<R, W>(mut reader: R, mut writer: W) -> Result<(), FilterError>
where
    R: BufRead,
    W: Write,
{
    let mut buf = vec![];
    while let Ok(n) = reader.read_until(b'\n', &mut buf) {
        if n > 0 {
            writer.write_all(&buf)?;
            buf.clear();
        } else {
            break;
        }
    }
    Ok(())
}

/// 1. Read a line as bytes
/// 2. Run f with bytes with new-line code
/// 3. Write the result of 2
pub fn run_per_line_as_byte<R, W, F>(mut reader: R, mut writer: W, mut f: F) -> Result<(), FilterError>
where
    F: FnMut(&[u8]) -> Result<Vec<u8>, FilterError>,
    R: BufRead,
    W: Write,
{
    let mut buf = vec![];
    while let Ok(n) = reader.read_until(b'\n', &mut buf) {
        if n > 0 {
            let out = f(&buf)?;
            writer.write_all(&out)?;
            buf.clear();
        } else {
            break;
        }
    }
    Ok(())
}
