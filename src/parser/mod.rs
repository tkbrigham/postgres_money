use regex::Regex;

pub mod error;
pub use self::error::Error;

use crate::Money;

fn parse_en_us_utf8(input: &str) -> Result<Money, Error> {
    money_str_to_int(input).map(Money)
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

impl Money {
    pub fn parse_str(input: &str) -> Result<Money, Error> {
        parse_en_us_utf8(&input.replace("$", ""))
    }

    // TODO
    // pub fn parse_int<T>(input: T) -> Result<Money, Error> {
    // }
}

type Inner = String;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd, Debug)]
enum AmountKind {
    Negative,
    Positive,
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd, Debug)]
struct Amount {
    kind: AmountKind,
    inner: Inner
}

impl Amount {
    fn new(kind: AmountKind, inner: &str) -> Result<Self, Error> {
        return if Self::valid_inner().is_match(inner) {
            Ok(Amount {
                kind,
                inner: inner.to_string()
            })
        } else {
            Err(Error::InvalidString)
        }
    }

    pub fn positive(s: &str) -> Result<Amount, Error> {
        Self::new(AmountKind::Positive, s)
    }

    pub fn negative(s: &str) -> Result<Amount, Error> {
        Self::new(AmountKind::Negative, s)
    }

    fn valid_inner() -> Regex {
        Regex::new(r"^\$?(?P<dollars>\d*)\.?(?P<cents>\d*$)").unwrap()
    }

    fn from(s: &str) -> Result<Self, Error> {
        let has_minus = Regex::new(r"-(.*)").unwrap();
        let has_paren = Regex::new(r"\((.*)\)").unwrap();

        let m: Vec<Regex> = vec![has_minus, has_paren].into_iter()
            .filter(|r| r.is_match(s))
            .collect();

        return match m.len() {
            0 => Self::positive(s),
            1 => {
                let transformed = m.into_iter()
                    .fold(s, |s, r| r.captures(s).unwrap().get(1).unwrap().as_str());
                Self::negative(transformed)
            },
            _ => Err(Error::InvalidString)
        }
    }

    // fn to_money(self) -> Money {
    //
    // }

    // fn combine_dollars_and_cents(&self) -> i64 {
    //
    // }

    // fn get_parts(self) {
    //     let caps = Self::valid_inner().captures(self.0).unwrap();
    //
    //     let dollars = caps.get(1).map_or("", |m| m.as_str());
    //     let cents = caps.get(2).map_or("", |m| m.as_str());
    //
    //     println!("dollars = {:?}", dollars);
    //     println!("cents = {:?}", cents);
    //     // (dollars, cents)
    // }
}

// expects no currency symbols
fn money_str_to_int(input: &str) -> Result<i64, Error> {
    let money = Regex::new(r"^(\d*\.?\d*)").unwrap();
    match money.find(input) {
        Some(m) => mk_int(&m.as_str().replace(".", "")),
        None => Err(Error::InvalidString)
    }
}

fn mk_int(s: &str) -> Result<i64, Error> {
    str::parse::<i64>(&s).map_err(|_e| Error::ParseInt)
}

// fn cents_int_for(s: &str) -> i8 {
//
// }



#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::error::Error::InvalidString;

    #[test]
    fn test_goof() {
        let a = Amount::positive("100").unwrap();
        println!("{}", a.inner);
    }

    // #[test]
    // fn test_amt_pos() {
    //     let a = Amount::from("100");
    //     assert_eq!(a, Ok(Amount::positive("100")));
    // }
    //
    // #[test]
    // fn test_amt_pos_2() {
    //     let a = Amount::from("$100");
    //     assert_eq!(a, Ok(Amount::positive("$100")));
    // }
    //
    // #[test]
    // fn test_amt_invalid() {
    //     let a = Amount::from("-($100)");
    //     assert!(a.is_err());
    // }
    //
    // #[test]
    // fn test_amt_neg_paren() {
    //     let a = Amount::from("-$100")?;
    //     assert_eq!(a, Amount::negative("$100"));
    // }
    //
    // #[test]
    // fn test_wacky() {
    //     let a = Amount::from("-$100-$100");
    //     assert_eq!(a, Err(Error::InvalidString));
    // }

    // #[test]
    // fn test_amt_neg_minus() {
    //     let a = Amount::from("($100)");
    //     assert_eq!(a, Ok(Amount::Negative("100".to_string())));
    // }

    // #[test]
    // fn test_neg_parts() {
    //     let s = "-1000.0";
    //
    //     get_parts(s);
    // }
    //
    //
    // #[test]
    // fn test_valid_123_45() {
    //     assert_eq!(Money::parse_str("$123.45"), Ok(Money(12345)))
    // }
    //
    // #[test]
    // fn test_valid_123_451() {
    //     assert_eq!(Money::parse_str("$123.451"), Ok(Money(12345)))
    // }
    //
    // #[test]
    // fn test_valid_123_454() {
    //     assert_eq!(Money::parse_str("$123.454"), Ok(Money(12345)))
    // }
    //
    // #[test]
    // fn test_valid_123_455() {
    //     assert_eq!(Money::parse_str("$123.455"), Ok(Money(12346)))
    // }
    //
    // #[test]
    // fn test_valid_123_456() {
    //     assert_eq!(Money::parse_str("$123.456"), Ok(Money(12346)))
    // }
    //
    // #[test]
    // fn test_valid_123_459() {
    //     assert_eq!(Money::parse_str("$123.459"), Ok(Money(12346)))
    // }
    //
    // #[test]
    // fn test_valid_1234567890() {
    //     assert_eq!(Money::parse_str("1234567890"), Ok(Money(123456789000)))
    // }
    //
    // #[test]
    // fn test_valid_12345678901234567() {
    //     assert_eq!(Money::parse_str("12345678901234567"), Ok(Money(1234567890123456700)))
    // }
    //
    // #[test]
    // fn test_invalid_123456789012345678() {
    //     assert_eq!(Money::parse_str("123456789012345678"), Err(Error::OutOfRange))
    // }
    //
    // #[test]
    // fn test_invalid_9223372036854775807() {
    //     assert_eq!(Money::parse_str("9223372036854775807"), Err(Error::OutOfRange))
    // }

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
