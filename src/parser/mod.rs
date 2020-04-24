pub(crate) mod error;
pub(crate) use self::error::Error;

use crate::Money;
use std::num::ParseIntError;

fn parse_en_us_utf8(input: &str) -> Result<Money, crate::Error> {
    let inner = str::parse::<i64>(input)?;
    Ok(Money(inner))
}

impl Money {
    pub fn parse_str(input: &str) -> Result<Money, crate::Error> {
        parse_en_us_utf8(&str)
    }

    // TODO
    // pub fn parse_int<T>(input: T) -> Result<Money, crate::Error> {
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::num::dec2flt::parse::ParseResult::Invalid;
    use crate::parser::error::Error::OutOfRange;

    #[test]
    fn test_valid_123_45() {
        assert_eq!(Money::parse_str("$123.45"), Money(12345))
    }

    #[test]
    fn test_valid_123_451() {
        assert_eq!(Money::parse_str("$123.451"), Money(12345))
    }

    #[test]
    fn test_valid_123_454() {
        assert_eq!(Money::parse_str("$123.454"), Money(12345))
    }

    #[test]
    fn test_valid_123_455() {
        assert_eq!(Money::parse_str("$123.455"), Money(12346))
    }

    #[test]
    fn test_valid_123_456() {
        assert_eq!(Money::parse_str("$123.456"), Money(12346))
    }

    #[test]
    fn test_valid_123_459() {
        assert_eq!(Money::parse_str("$123.459"), Money(12346))
    }

    #[test]
    fn test_valid_1234567890() {
        assert_eq!(Money::parse_str("1234567890"), Money(123456789000))
    }

    #[test]
    fn test_valid_12345678901234567() {
        assert_eq!(Money::parse_str("12345678901234567"), Money(1234567890123456700))
    }

    #[test]
    fn test_invalid_123456789012345678() {
        assert_eq!(Money::parse_str("123456789012345678"), crate::Error(OutOfRange))
    }

    #[test]
    fn test_invalid_9223372036854775807() {
        assert_eq!(Money::parse_str("9223372036854775807"), crate::Error(OutOfRange))
    }

    #[test]
    fn test_valid_neg_12345() {
        assert_eq!(Money::parse_str("-12345"), Money(-1234500))
    }

    #[test]
    fn test_valid_neg_1234567890() {
        assert_eq!(Money::parse_str("-1234567890"), Money(-123456789000))
    }

    #[test]
    fn test_valid_neg_12345678901234567() {
        assert_eq!(Money::parse_str("-12345678901234567"), Money(-1234567890123456700))
    }

    #[test]
    fn test_invalid_neg_123456789012345678() {
        assert_eq!(Money::parse_str("-123456789012345678"), crate::Error(OutOfRange))
    }

    #[test]
    fn test_invalid_neg_9223372036854775808() {
        assert_eq!(Money::parse_str("-9223372036854775808"), crate::Error(OutOfRange))
    }

    #[test]
    fn test_valid_paren_1() {
        assert_eq!(Money::parse_str("(1)"), Money(-100))
    }

    #[test]
    fn test_valid_paren_123456_78() {
        assert_eq!(Money::parse_str("($123,456.78)"), Money(-12345678))
    }

    #[test]
    fn test_valid_paren_123456_78() {
        assert_eq!(Money::parse_str("($123,456.78)"), Money(-12345678))
    }

    #[test]
    fn test_valid_min() {
        assert_eq!(Money::parse_str("-92233720368547758.08"), Money::min())
    }

    #[test]
    fn test_valid_max() {
        assert_eq!(Money::parse_str("92233720368547758.07"), Money::max())
    }

    #[test]
    fn test_invalid_min() {
        assert_eq!(Money::parse_str("-92233720368547758.085"), crate::Error(OutOfRange))
    }

    #[test]
    fn test_invalid_max() {
        assert_eq!(Money::parse_str("92233720368547758.075"), crate::Error(OutOfRange))
    }


    // TODO: int parsing

    // TODO: https://github.com/postgres/postgres/blob/master/src/test/regress/sql/money.sql
    // TODO: https://github.com/postgres/postgres/blob/master/src/test/regress/expected/money.out

    // #[test]
    // fn test_parse_str_neg_no_decimal() {
    //     assert!(Money::parse_str("-300").is_ok());
    // }
    //
    // #[test]
    // fn test_parse_str_neg_with_decimal() {
    //     assert!(Money::parse_str("-300.01").is_ok());
    // }
    //
    // #[test]
    // fn test_parse_str_pos_no_decimal() {
    //     assert!(Money::parse_str("302").is_ok());
    // }
    //
    // #[test]
    // fn test_parse_str_pos_with_decimal() {
    //     assert!(Money::parse_str("302.01").is_ok());
    // }
}
