use std::io::{BufRead, Write};

/// 1. Read all bytes as UTF-8 String
/// 2. Run f with String
/// 3. Write the result of 2
pub fn run_all_input<R, W, F>(mut reader: R, mut writer: W, f: F)
where
    F: FnOnce(&str) -> String,
    R: BufRead,
    W: Write,
{
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();

    let out = f(&buffer);
    writer.write_all(out.as_bytes()).unwrap();
}

pub fn do_nothing<R, W>(mut reader: R, mut writer: W)
where
    R: BufRead,
    W: Write,
{
    let mut buf = vec![];
    while let Ok(n) = reader.read_until(b'\n', &mut buf) {
        if n > 0 {
            writer.write_all(&buf).unwrap();
            buf.clear();
        } else {
            break;
        }
    }
}

/// 1. Read a line as bytes
/// 2. Run f with bytes with new-line code
/// 3. Write the result of 2
pub fn run_per_line_as_byte<R, W, F>(mut reader: R, mut writer: W, mut f: F)
where
    F: FnMut(&[u8]) -> Vec<u8>,
    R: BufRead,
    W: Write,
{
    let mut buf = vec![];
    while let Ok(n) = reader.read_until(b'\n', &mut buf) {
        if n > 0 {
            let out = f(&buf);
            writer.write_all(&out).unwrap();
            buf.clear();
        } else {
            break;
        }
    }
}
// fn run_per_line<R, W, F>(reader: R, mut writer: W, mut f: F)
// where
//     F: FnMut(&str) -> String,
//     R: BufRead,
//     W: Write,
// {
//     for line in reader.lines() {
//         let out = f(&line.unwrap());
//         writer.write_all(out.as_bytes()).unwrap();
//         writer.flush().unwrap();
//     }
// }
