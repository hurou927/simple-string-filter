#[cfg(test)]
mod tests {

    mod json_encode_tests {
        use super::super::super::json::*;
        use rstest::rstest;

        #[rstest]
        #[case("abc", r#""abc""#)]
        #[case("a\"c", r#""a\"c""#)]
        #[case("a\nc", r#""a\nc""#)]
        fn should_encode_simple_string(#[case] input: &str, #[case] expected: &str) {
            let is_raw = false;
            let f = JsonEncodeFilter::new(is_raw);
            let actual = f.run(input);
            assert_eq!(expected, actual.unwrap());
        }
        #[rstest]
        #[case("abc", r#"abc"#)]
        #[case("a\"c", r#"a\"c"#)]
        #[case("a\nc", r#"a\nc"#)]
        fn should_encode_simple_string_with_raw(#[case] input: &str, #[case] expected: &str) {
            let is_raw = true;
            let f = JsonEncodeFilter::new(is_raw);
            let actual = f.run(input);
            assert_eq!(expected, actual.unwrap());
        }

    }

    mod json_decode_tests {
        use super::super::super::json::*;
        use rstest::rstest;

        #[rstest]
        #[case(r#""abc""#, "abc")]
        #[case(r#""a\"c""#, "a\"c" )]
        #[case(r#""a\nc""#,"a\nc" )]
        fn should_decode_simple_string(#[case] input: &str, #[case] expected: &str) {
            let is_raw = false;
            let f = JsonDecodeFilter::new(is_raw);
            let actual = f.run(input);
            assert_eq!(expected, actual.unwrap());
        }

        #[rstest]
        #[case(r#"abc"#, "abc")]
        #[case(r#"a\"c"#, "a\"c" )]
        #[case(r#"a\nc"#,"a\nc" )]
        fn should_decode_simple_string_with_raw(#[case] input: &str, #[case] expected: &str) {
            let is_raw = true;
            let f = JsonDecodeFilter::new(is_raw);
            let actual = f.run(input);
            assert_eq!(expected, actual.unwrap());
        }
    }
}
