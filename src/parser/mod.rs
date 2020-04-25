use regex::Regex;

pub mod error;
pub use self::error::Error;

use crate::{Money, Inner};

fn parse_en_us_utf8(input: &str) -> Result<Money, Error> {
    // let (dollars, cents): (i64, i64) = input.replace("$", "")
    //     .split(".")
    //     .collect::<Vec<&str>>()
    //     .map(|section| str::parse::<i64>(section))
    //     .map_err(|_ignore| return Error::ParseInt)?;
    //
    // println!("dollars: {}; cents: {}", dollars, cents);
    //
    let inner = 8080;
    Ok(Money(inner))
}

// fn convert_to_cents(input: &str) -> Result<Inner, Error> {
//     let (dollars, cents): (i64, i64) = input
//         .replace("$", "")
//
//         .split(".")
// }
//
// fn extract_amount(input: &str) -> Amount {
// }

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Amount {
    Negative(String),
    Positive(String)
}

impl Money {
    pub fn parse_str(input: &str) -> Result<Money, Error> {
        parse_en_us_utf8(input)
    }

    // TODO
    // pub fn parse_int<T>(input: T) -> Result<Money, Error> {
    // }
}

fn unwrap_minus(input: &str) -> Option<Amount> {
    let neg = Regex::new(r"^-\$(?P<cash_neg>[\d\.]*)").unwrap();
    for cap in neg.captures_iter(input) {
        return Some(Amount::Negative(cap["cash_neg"].to_string()))
    }
    None
}

// fn unwrap_paren(input: &str) -> Option<Amount> {
//
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::error::Error::OutOfRange;

    #[test]
    fn test_unwrap_minus_is_none_when_paren_and_neg() {
        assert!(unwrap_minus("(-$100.00)").is_none());
    }

    #[test]
    fn test_unwrap_minus_is_none_when_neg_and_paren() {
        assert!(unwrap_minus("-($100.00)").is_none());
    }

    #[test]
    fn test_unwrap_minus_is_none_when_unrecognized_currency() {
        assert!(unwrap_minus("-¥100.00)").is_none());
    }

    #[test]
    fn test_unwrap_minus_unwraps() {
        let expected = Amount::Negative("100.00".to_string());
        match unwrap_minus("-$100.00") {
            Some(negative) => assert_eq!(negative, expected),
            None => assert!(false, "expected a Some")
        }
    }

    #[test]
    fn test_neg_paren() {
        let paren = Regex::new(r"^\(\$?(?P<cash_paren>[\d\.]*)\)").unwrap();
        let caps = paren.captures_iter("(-$100.00)");

        for cap in caps {
            println!("paren: {:?}", &cap["cash_paren"])
        }
    }

    // #[test]
    // fn test_dummy() {
    //     // let t = "92233720368547758078";
    //     // let res= str::parse::<i64>(t);
    //     // println!("{:?}", res);
    //
    //     // let re = Regex::new(r"\(+\$+\d*\.+\d*\)+").unwrap();
    //     let valid = Regex::new(r"-\(+\$+(\d+)\.+(\d+)\)+").unwrap();
    //     // let re = Regex::new(r"(\d+)\.+(\d+)").unwrap();
    //     let caps = valid.captures("($100.00)").unwrap();
    //     println!("{:?}", caps.get(0).unwrap().as_str());
    //     println!("{:?}", caps.get(1).unwrap().as_str());
    //     println!("{:?}", caps.get(2).unwrap().as_str());
    //     return
    //     // assert!(str::parse::<i64>(t).is_ok())
    // }

    // #[test]
    // fn test_valid_123_45() {
    //     assert_eq!(Money::parse_str("$123.45"), Ok(Money(12345)))
    // }
    //
    // #[test]
    // fn test_valid_123_451() {
    //     assert_eq!(Money::parse_str("$123.451"), Money(12345))
    // }
    //
    // #[test]
    // fn test_valid_123_454() {
    //     assert_eq!(Money::parse_str("$123.454"), Money(12345))
    // }
    //
    // #[test]
    // fn test_valid_123_455() {
    //     assert_eq!(Money::parse_str("$123.455"), Money(12346))
    // }
    //
    // #[test]
    // fn test_valid_123_456() {
    //     assert_eq!(Money::parse_str("$123.456"), Money(12346))
    // }
    //
    // #[test]
    // fn test_valid_123_459() {
    //     assert_eq!(Money::parse_str("$123.459"), Money(12346))
    // }
    //
    // #[test]
    // fn test_valid_1234567890() {
    //     assert_eq!(Money::parse_str("1234567890"), Money(123456789000))
    // }
    //
    // #[test]
    // fn test_valid_12345678901234567() {
    //     assert_eq!(Money::parse_str("12345678901234567"), Money(1234567890123456700))
    // }
    //
    // #[test]
    // fn test_invalid_123456789012345678() {
    //     assert_eq!(Money::parse_str("123456789012345678"), Error(OutOfRange))
    // }
    //
    // #[test]
    // fn test_invalid_9223372036854775807() {
    //     assert_eq!(Money::parse_str("9223372036854775807"), Error(OutOfRange))
    // }
    //
    // #[test]
    // fn test_valid_neg_12345() {
    //     assert_eq!(Money::parse_str("-12345"), Money(-1234500))
    // }
    //
    // #[test]
    // fn test_valid_neg_1234567890() {
    //     assert_eq!(Money::parse_str("-1234567890"), Money(-123456789000))
    // }
    //
    // #[test]
    // fn test_valid_neg_12345678901234567() {
    //     assert_eq!(Money::parse_str("-12345678901234567"), Money(-1234567890123456700))
    // }
    //
    // #[test]
    // fn test_invalid_neg_123456789012345678() {
    //     assert_eq!(Money::parse_str("-123456789012345678"), Error(OutOfRange))
    // }
    //
    // #[test]
    // fn test_invalid_neg_9223372036854775808() {
    //     assert_eq!(Money::parse_str("-9223372036854775808"), Error(OutOfRange))
    // }
    //
    // #[test]
    // fn test_valid_paren_1() {
    //     assert_eq!(Money::parse_str("(1)"), Money(-100))
    // }
    //
    // #[test]
    // fn test_valid_paren_123456_78() {
    //     assert_eq!(Money::parse_str("($123,456.78)"), Money(-12345678))
    // }
    //
    // #[test]
    // fn test_valid_paren_123456_78() {
    //     assert_eq!(Money::parse_str("($123,456.78)"), Money(-12345678))
    // }
    //
    // #[test]
    // fn test_valid_min() {
    //     assert_eq!(Money::parse_str("-92233720368547758.08"), Money::min())
    // }
    //
    // #[test]
    // fn test_valid_max() {
    //     assert_eq!(Money::parse_str("92233720368547758.07"), Money::max())
    // }
    //
    // #[test]
    // fn test_invalid_min() {
    //     assert_eq!(Money::parse_str("-92233720368547758.085"), Error(OutOfRange))
    // }
    //
    // #[test]
    // fn test_invalid_max() {
    //     assert_eq!(Money::parse_str("92233720368547758.075"), Error(OutOfRange))
    // }

    // TODO: int parsing

    // TODO: https://github.com/postgres/postgres/blob/master/src/test/regress/sql/money.sql
    // TODO: https://github.com/postgres/postgres/blob/master/src/test/regress/expected/money.out
}
