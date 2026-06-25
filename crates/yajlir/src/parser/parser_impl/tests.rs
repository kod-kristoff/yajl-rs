use super::parse_integer;

mod parse_integer {
    use crate::parser::ParseIntegerError;

    use super::parse_integer;
    use rstest::rstest;

    #[rstest]
    #[case(&b"0"[..],0)]
    #[case(&b"123456789"[..],123456789)]
    #[case(&b"123456789123456789"[..],123456789123456789)]
    #[case(&b"-123456789"[..],-123456789)]
    #[case(&b"-123456789123456789"[..],-123456789123456789)]
    #[case(&b"-0"[..],0)]
    fn successful(#[case] input: &[u8], #[case] expected: i64) {
        let res = unsafe { parse_integer(input.as_ptr(), input.len()) };

        assert_eq!(res.unwrap(), expected);
    }

    #[rstest]
    #[case(&b"0a"[..],ParseIntegerError::NonNumerical(b'a'))]
    #[case(&b"123456789123456789123456789"[..],ParseIntegerError::Overflow)]
    #[case(&b"-123456789123456789123456789"[..],ParseIntegerError::Underflow)]
    // #[case(&b"123456789"[..],123456789)]
    // #[case(&b"-0"[..],0)]
    fn failure(#[case] input: &[u8], #[case] expected: ParseIntegerError) {
        let res = unsafe { parse_integer(input.as_ptr(), input.len()) };

        assert_eq!(res.unwrap_err(), expected);
    }
}
