use regex::{Regex, Match};

pub mod error;
pub use self::error::Error;

use crate::Money;

fn parse_en_us_utf8(input: &str) -> Result<Money, Error> {
    Amount::from(input)?.to_money()
}

impl Money {
    pub fn parse_str(input: &str) -> Result<Money, Error> {
        parse_en_us_utf8(input)
    }

    // TODO
    // pub fn parse_int<T>(input: T) -> Result<Money, Error> {
    // }
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd, Debug)]
enum AmountKind {
    Negative,
    Positive,
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd, Debug)]
struct Amount {
    kind: AmountKind,
    dollars: String,
    cents: String,
}

impl Amount {
    fn new(kind: AmountKind, inner: &str) -> Result<Self, Error> {
        let caps = Self::valid_inner()
            .captures(inner)
            .ok_or(Error::InvalidString)?;

        if caps.len() != 3 {
            return Err(Error::InvalidString)
        }

        Ok(Amount {
            kind,
            dollars: Self::mk_string(caps.get(1)),
            cents: Self::mk_string(caps.get(2)),
        })
    }

    fn positive(s: &str) -> Result<Amount, Error> {
        Self::new(AmountKind::Positive, s)
    }

    fn negative(s: &str) -> Result<Amount, Error> {
        Self::new(AmountKind::Negative, s)
    }

    fn valid_inner() -> Regex {
        Regex::new(r"^\$?(?P<dollars>\d*)\.?(?P<cents>\d*$)").unwrap()
    }

    fn mk_string(m: Option<Match>) -> String {
        m.map_or("", |m| m.as_str()).to_string()
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

    fn to_money(&self) -> Result<Money, Error> {
        let inner = self.combine_dollars_and_cents()?;
        Ok(Money(inner))
    }

    fn combine_dollars_and_cents(&self) -> Result<i64, Error> {
        let dollars = mk_int(&self.dollars)?;
        let cents = mk_rounded_cents(&self.cents)?;
        let tot = dollars.checked_mul(100)
            .ok_or(Error::OutOfRange)?
            .checked_add(cents)
            .ok_or(Error::OutOfRange);

        return if &self.kind == &AmountKind::Negative {
            tot?.checked_mul(-1).ok_or(Error::OutOfRange)
        } else {
            tot
        };
    }
}

fn mk_rounded_cents(s: &String) -> Result<i64, Error> {
    return if s.len() > 2 {
        round_cents(s)
    } else {
        mk_int(s)
    };
}

fn round_cents(s: &String) -> Result<i64, Error> {
    let s = &s[..3];
    let (s1, s2) = s.split_at(s.len() - 1);
    let (i1, i2) = (mk_int(s1)?, mk_int(s2)?);
    if i2 >= 5 {
        Ok(i1 + 1)
    } else {
        Ok(i1)
    }
}

fn mk_int(s: &str) -> Result<i64, Error> {
    if s.is_empty() {
        return Ok(0)
    }

    str::parse::<i64>(&s)
        .map_err(|e| {
            // This is a janky workaround until ParseIntError.kind() is stable
            match e.to_string().find("too large") {
                Some(_) => Error::OutOfRange,
                None => Error::ParseInt
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_str_to_round() {
    //     let s = "145999";
    //     let ret = mk_rounded_cents(&s.to_string()).expect("oh yeah");
    //     println!("ret = {}", ret);
    // }
    //
    // #[test]
    // fn test_amt_pos() {
    //     let a = Amount::from("100").unwrap();
    //     assert_eq!(a.kind, AmountKind::Positive);
    //     assert_eq!(a.dollars, "100");
    //     assert_eq!(a.cents, "");
    // }
    //
    // #[test]
    // fn test_amt_neg() {
    //     let a = Amount::from("-100").unwrap();
    //     assert_eq!(a.kind, AmountKind::Negative);
    //     assert_eq!(a.dollars, "100");
    //     assert_eq!(a.cents, "");
    // }
    //
    // #[test]
    // fn test_amt_zero() {
    //     let a = Amount::from("0").unwrap();
    //     assert_eq!(a.kind, AmountKind::Positive);
    //     assert_eq!(a.dollars, "0");
    //     assert_eq!(a.cents, "");
    // }
    //
    // #[test]
    // fn test_amt_dollars_cents() {
    //     let a = Amount::from("9.8").unwrap();
    //     assert_eq!(a.kind, AmountKind::Positive);
    //     assert_eq!(a.dollars, "9");
    //     assert_eq!(a.cents, "8");
    // }
    //
    // #[test]
    // fn test_amt_dollars_cents_neg() {
    //     let a = Amount::from("-9.8").unwrap();
    //     assert_eq!(a.kind, AmountKind::Negative);
    //     assert_eq!(a.dollars, "9");
    //     assert_eq!(a.cents, "8");
    // }
    //
    // #[test]
    // fn test_amt_dollars_cents_neg_unrounded() {
    //     let a = Amount::from("-9.8023").unwrap();
    //     assert_eq!(a.kind, AmountKind::Negative);
    //     assert_eq!(a.dollars, "9");
    //     assert_eq!(a.cents, "8023");
    // }
    //
    // #[test]
    // fn test_invalid() {
    //     let a = Amount::from("-(100)");
    //     assert_eq!(a, Err(Error::InvalidString));
    // }
    //
    // #[test]
    // fn test_invalid_garbage() {
    //     let a = Amount::from("100b");
    //     assert_eq!(a, Err(Error::InvalidString));
    // }

    // TODO: actual tests
    #[test]
    fn test_valid_123_45() {
        assert_eq!(Money::parse_str("$123.45"), Ok(Money(12345)))
    }

    #[test]
    fn test_valid_123_451() {
        assert_eq!(Money::parse_str("$123.451"), Ok(Money(12345)))
    }

    #[test]
    fn test_valid_123_454() {
        assert_eq!(Money::parse_str("$123.454"), Ok(Money(12345)))
    }

    #[test]
    fn test_valid_123_455() {
        assert_eq!(Money::parse_str("$123.455"), Ok(Money(12346)))
    }

    #[test]
    fn test_valid_123_456() {
        assert_eq!(Money::parse_str("$123.456"), Ok(Money(12346)))
    }

    #[test]
    fn test_valid_123_459() {
        assert_eq!(Money::parse_str("$123.459"), Ok(Money(12346)))
    }

    #[test]
    fn test_valid_1234567890() {
        assert_eq!(Money::parse_str("1234567890"), Ok(Money(123456789000)))
    }

    #[test]
    fn test_valid_12345678901234567() {
        assert_eq!(Money::parse_str("12345678901234567"), Ok(Money(1234567890123456700)))
    }

    #[test]
    fn test_invalid_123456789012345678() {
        assert_eq!(Money::parse_str("123456789012345678"), Err(Error::OutOfRange))
    }

    #[test]
    fn test_invalid_9223372036854775807() {
        assert_eq!(Money::parse_str("9223372036854775807"), Err(Error::OutOfRange))
    }

    #[test]
    fn test_valid_neg_12345() {
        assert_eq!(Money::parse_str("-12345"), Ok(Money(-1234500)))
    }

    // TKB CURRENT
    #[test]
    fn test_valid_neg_1234567890() {
        assert_eq!(Money::parse_str("-1234567890"), Ok(Money(-123456789000)))
    }

    #[test]
    fn test_valid_neg_12345678901234567() {
        assert_eq!(Money::parse_str("-12345678901234567"), Ok(Money(-1234567890123456700)))
    }

    #[test]
    fn test_invalid_neg_123456789012345678() {
        assert_eq!(Money::parse_str("-123456789012345678"), Err(Error::OutOfRange))
    }

    #[test]
    fn test_invalid_neg_9223372036854775808() {
        assert_eq!(Money::parse_str("-9223372036854775808"), Err(Error::OutOfRange))
    }

    // #[test]
    // fn test_valid_paren_1() {
    //     assert_eq!(Money::parse_str("(1)"), Ok(Money(-100)))
    // }
    //
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
