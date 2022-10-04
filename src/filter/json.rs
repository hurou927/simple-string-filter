use crate::core::error::FilterError;


#[derive(Debug)]
pub struct JsonEncodeFilter {
    is_raw: bool,
}

impl JsonEncodeFilter {
    pub fn new(is_raw: bool) -> Self {
        JsonEncodeFilter { is_raw }
    }

    pub fn run(&self, buffer: &str) -> Result<String, FilterError> {
        let mut json_encoded_string = serde_json::to_string_pretty(buffer)?;
        if self.is_raw {
            // remove first and last double-quote
            json_encoded_string.pop();
            json_encoded_string.remove(0);
        }
        Ok(json_encoded_string)
    }
}

#[derive(Debug)]
pub struct JsonDecodeFilter {
    is_raw: bool,
}

impl JsonDecodeFilter {
    pub fn new(is_raw: bool) -> Self {
        JsonDecodeFilter { is_raw }
    }
    pub fn run(&self, buffer: &str) -> Result<String, FilterError> {
            // https://www.rfc-editor.org/rfc/rfc8259.html
            let out = if self.is_raw {
               let new_buf = "\"".to_owned() + buffer + "\"";
                serde_json::from_str(&new_buf)?
            } else {
                serde_json::from_str(buffer)?
            };
            Ok(out)
    }
}
