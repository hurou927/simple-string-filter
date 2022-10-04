use crate::core::error::FilterError;


pub struct JsonEncoder {
    is_raw: bool,
}

impl JsonEncoder {
    pub fn new(is_raw: bool) -> Self {
        JsonEncoder { is_raw }
    }

    pub fn json_ecode(&self, buffer: &str) -> Result<String, FilterError> {
        let mut json_encoded_string = serde_json::to_string_pretty(buffer)?;
        if self.is_raw {
            // remove first and last double-quote
            json_encoded_string.pop();
            json_encoded_string.remove(0);
        }
        Ok(json_encoded_string)
    }
}

pub struct JsonDecoder {
    is_raw: bool,
}

impl JsonDecoder {
    pub fn new(is_raw: bool) -> Self {
        JsonDecoder { is_raw }
    }
    pub fn json_decode(&self, buffer: &str) -> Result<String, FilterError> {
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
