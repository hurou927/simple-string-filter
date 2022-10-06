use crate::core::error::FilterError;

#[derive(Debug)]
pub struct ToLfFilter;

impl ToLfFilter {
    pub fn run(&self, buffer: &[u8]) -> Result<Vec<u8>, FilterError> {
        let mut new_buffer = buffer.to_owned();
        if new_buffer.ends_with(&[b'\n']) {
            new_buffer.pop();
            if new_buffer.ends_with(&[b'\r']) {
                new_buffer.pop();
            }
            new_buffer.push(b'\n');
        }
        Ok(new_buffer)
    }
}

#[derive(Debug)]
pub struct ToCrLfFilter;

impl ToCrLfFilter {
    pub fn run(&self, buffer: &[u8]) -> Result<Vec<u8>, FilterError> {
        let mut new_buffer = buffer.to_owned();
        if new_buffer.ends_with(&[b'\n']) {
            new_buffer.pop();
            if new_buffer.ends_with(&[b'\r']) {
                new_buffer.pop();
            }
            new_buffer.push(b'\r');
            new_buffer.push(b'\n');
        }
        Ok(new_buffer)
    }
}
