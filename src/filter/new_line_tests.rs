#[cfg(test)]
mod tests {

    mod json_encode_tests {
        use super::super::super::new_line::*;
        use rstest::rstest;

        #[rstest]
        #[case("abc\n", "abc\n")]
        #[case("abc\r\n", "abc\n")]
        #[case("abc", "abc")]
        fn should_return_lf(#[case] input: &str, #[case] expected: &str) {
            let actual = ToLfFilter::new().run(input.as_bytes());
            assert_eq!(expected.as_bytes(), actual.unwrap());
        }

        #[rstest]
        #[case("abc\n", "abc\r\n")]
        #[case("abc\r\n", "abc\r\n")]
        #[case("abc", "abc")]
        fn should_return_crlf(#[case] input: &str, #[case] expected: &str) {
            let actual = ToCrLfFilter::new().run(input.as_bytes());
            assert_eq!(expected.as_bytes(), actual.unwrap());
        }
    }
}
