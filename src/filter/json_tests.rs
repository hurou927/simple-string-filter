#[cfg(test)]
mod tests {

    mod json_encode_tests {
        use super::super::super::json::*;

        #[test]
        fn should_encode_simple_string() {
            let is_raw = false;
            let f = JsonEncodeFilter::new(is_raw);

            let input = "Rust is a systems programming language focused on three
goals: safety, speed, and concurrency.";
            let expected = r#""Rust is a systems programming language focused on three\ngoals: safety, speed, and concurrency.""#;
            let actual = f.run(input);
            assert_eq!(actual.unwrap(), expected);
        }

        #[test]
        fn should_encode_simple_string_without_double_quote() {
            let is_raw = true;
            let f = JsonEncodeFilter::new(is_raw);

            let input = "Rust is a systems programming language focused on three
goals: safety, speed, and concurrency.";
            let expected = r#"Rust is a systems programming language focused on three\ngoals: safety, speed, and concurrency."#;
            let actual = f.run(input);
            assert_eq!(actual.unwrap(), expected);
        }
    }

    mod json_decode_tests {
        use super::super::super::json::*;

        #[test]
        fn should_decode_simple_string() {
            let is_raw = false;
            let f = JsonDecodeFilter::new(is_raw);

            let input = r#""Rust is a systems programming language focused on three\ngoals: safety, speed, and concurrency.""#;
            let expected = "Rust is a systems programming language focused on three
goals: safety, speed, and concurrency.";
            let actual = f.run(input);
            assert_eq!(actual.unwrap(), expected);
        }

        #[test]
        fn should_decode_simple_string_without_double_quote() {
            let is_raw = true;
            let f = JsonDecodeFilter::new(is_raw);

            let input = r#"Rust is a systems programming language focused on three\ngoals: safety, speed, and concurrency."#;
            let expected = "Rust is a systems programming language focused on three
goals: safety, speed, and concurrency.";
            let actual = f.run(input);
            assert_eq!(actual.unwrap(), expected);
        }
    }
}
