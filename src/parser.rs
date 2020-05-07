use regex::{Match, Regex};

/// #[doc(inline)]
pub use crate::error::Error;

use crate::Money;

impl Money {
    /// Attempt to parse a `&str` into a `Money`.
    ///
    /// NOTE: as of this writing, only the Postgres `en_US.UTF-8` locale is supported.
    ///
    ///
    ///
    /// For more information about the Postgres `money` type, please see
    /// [8.2. Monetary Types](https://www.postgresql.org/docs/9.1/datatype-money.html).
    ///
    /// For more about how locales work with monetary values, please see
    /// [lc_monetary](https://www.postgresql.org/docs/9.1/runtime-config-client.html#GUC-LC-MONETARY).
    ///
    /// # Examples
    /// Parse a string of dollars
    /// ```
    /// use postgres_money::Money;
    /// let dollars = "324023040222";
    /// let money = Money::parse_str(dollars).unwrap();
    ///
    /// assert_eq!("$324023040222.00", money.to_string());
    /// ```
    ///
    /// Parse a string of cents
    /// ```
    /// use postgres_money::Money;
    /// let dollars = ".32";
    /// let money = Money::parse_str(dollars).unwrap();
    ///
    /// assert_eq!("$0.32", money.to_string());
    /// ```
    ///
    /// Parse a string with dollars and cents
    /// ```
    /// use postgres_money::Money;
    /// let dollars = "93.32";
    /// let money = Money::parse_str(dollars).unwrap();
    ///
    /// assert_eq!("$93.32", money.to_string());
    /// ```
    ///
    /// Handles parentheses
    /// ```
    /// use postgres_money::Money;
    /// let dollars = "(93.32)";
    /// let money = Money::parse_str(dollars).unwrap();
    ///
    /// assert_eq!("-$93.32", money.to_string());
    /// ```
    ///
    /// Handles dollar symbols
    /// ```
    /// use postgres_money::Money;
    /// let dollars = "$93.32";
    /// let money = Money::parse_str(dollars).unwrap();
    ///
    /// assert_eq!("$93.32", money.to_string());
    /// ```
    ///
    /// Rounds correctly
    /// ```
    /// use postgres_money::Money;
    /// let dollars = "$123.454";
    /// let money = Money::parse_str(dollars).unwrap();
    ///
    /// assert_eq!("$123.45", money.to_string());
    ///
    /// let dollars = "$123.455";
    /// let money = Money::parse_str(dollars).unwrap();
    ///
    /// assert_eq!("$123.46", money.to_string());
    /// ```
    ///
    /// Handles commas
    /// ```
    /// use postgres_money::Money;
    /// let dollars = "$123,456.78";
    /// let money = Money::parse_str(dollars).unwrap();
    ///
    /// assert_eq!("$123456.78", money.to_string());
    /// ```
    ///
    /// Max value
    /// ```
    /// use postgres_money::Money;
    /// let dollars = "92233720368547758.07";
    /// let money = Money::parse_str(dollars).unwrap();
    ///
    /// assert_eq!("$92233720368547758.07", money.to_string());
    /// assert_eq!(Money::max().to_string(), money.to_string());
    /// ```
    ///
    /// Min value
    /// ```
    /// use postgres_money::Money;
    /// let dollars = "-92233720368547758.08";
    /// let money = Money::parse_str(dollars).unwrap();
    ///
    /// assert_eq!("-$92233720368547758.08", money.to_string());
    /// assert_eq!(Money::min().to_string(), money.to_string());
    /// ```
    pub fn parse_str(input: &str) -> Result<Money, Error> {
        parse_en_us_utf8(input)
    }

    /// Construct a Money instance from an i64
    /// # Examples
    /// ```
    /// use postgres_money::Money;
    /// let cents = 324023040222;
    /// let money = Money::from(cents);
    ///
    /// assert_eq!("$3240230402.22", money.to_string());
    /// ```
    pub fn from(cents: i64) -> Money {
        Money(cents)
    }
}

fn parse_en_us_utf8(input: &str) -> Result<Money, Error> {
    Amount::from(input)?.to_money()
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
            return Err(Error::InvalidString);
        }

        Ok(Amount {
            kind,
            dollars: Self::mk_string(caps.get(1)).replace(",", ""),
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
        Regex::new(r"^\$?(?P<dollars>[\d,]*)\.?(?P<cents>\d*$)").unwrap()
    }

    fn mk_string(m: Option<Match>) -> String {
        m.map_or("", |m| m.as_str()).to_string()
    }

    fn from(s: &str) -> Result<Self, Error> {
        let has_minus = Regex::new(r"-(.*)").unwrap();
        let has_paren = Regex::new(r"\((.*)\)").unwrap();

        let m: Vec<Regex> = vec![has_minus, has_paren]
            .into_iter()
            .filter(|r| r.is_match(s))
            .collect();

        return match m.len() {
            0 => Self::positive(s),
            1 => {
                let transformed = m
                    .into_iter()
                    .fold(s, |s, r| r.captures(s).unwrap().get(1).unwrap().as_str());
                Self::negative(transformed)
            }
            _ => Err(Error::InvalidString),
        };
    }

    fn to_money(&self) -> Result<Money, Error> {
        let inner = self.combine_dollars_and_cents()?;
        Ok(Money(inner))
    }

    fn apply_sign(&self) -> i64 {
        return if &self.kind == &AmountKind::Negative {
            -1
        } else {
            1
        };
    }

    fn combine_dollars_and_cents(&self) -> Result<i64, Error> {
        let dollars = mk_int(&self.dollars)? * self.apply_sign();
        let cents = mk_rounded_cents(&self.cents)? * self.apply_sign();

        dollars
            .checked_mul(100)
            .ok_or(Error::OutOfRange)?
            .checked_add(cents)
            .ok_or(Error::OutOfRange)
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
        return Ok(0);
    }

    str::parse::<i64>(&s).map_err(|e| {
        // This is a janky workaround until ParseIntError.kind() is stable
        match e.to_string().find("too large") {
            Some(_) => Error::OutOfRange,
            None => Error::ParseInt,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Money::parse_str
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
        assert_eq!(
            Money::parse_str("12345678901234567"),
            Ok(Money(1234567890123456700))
        )
    }

    #[test]
    fn test_invalid_123456789012345678() {
        assert_eq!(
            Money::parse_str("123456789012345678"),
            Err(Error::OutOfRange)
        )
    }

    #[test]
    fn test_invalid_9223372036854775807() {
        assert_eq!(
            Money::parse_str("9223372036854775807"),
            Err(Error::OutOfRange)
        )
    }

    #[test]
    fn test_valid_neg_12345() {
        assert_eq!(Money::parse_str("-12345"), Ok(Money(-1234500)))
    }

    #[test]
    fn test_valid_neg_1234567890() {
        assert_eq!(Money::parse_str("-1234567890"), Ok(Money(-123456789000)))
    }

    #[test]
    fn test_valid_neg_12345678901234567() {
        assert_eq!(
            Money::parse_str("-12345678901234567"),
            Ok(Money(-1234567890123456700))
        )
    }

    #[test]
    fn test_invalid_neg_123456789012345678() {
        assert_eq!(
            Money::parse_str("-123456789012345678"),
            Err(Error::OutOfRange)
        )
    }

    #[test]
    fn test_invalid_neg_9223372036854775808() {
        assert_eq!(
            Money::parse_str("-9223372036854775808"),
            Err(Error::OutOfRange)
        )
    }

    #[test]
    fn test_valid_paren_1() {
        assert_eq!(Money::parse_str("(1)"), Ok(Money(-100)))
    }

    #[test]
    fn test_valid_paren_123456_78() {
        assert_eq!(Money::parse_str("($123,456.78)"), Ok(Money(-12345678)))
    }

    #[test]
    fn test_valid_min() {
        assert_eq!(Money::parse_str("-92233720368547758.08"), Ok(Money::min()))
    }

    #[test]
    fn test_valid_max() {
        assert_eq!(Money::parse_str("92233720368547758.07"), Ok(Money::max()))
    }

    #[test]
    fn test_invalid_min() {
        assert_eq!(
            Money::parse_str("-92233720368547758.085"),
            Err(Error::OutOfRange)
        )
    }

    #[test]
    fn test_invalid_max() {
        assert_eq!(
            Money::parse_str("92233720368547758.075"),
            Err(Error::OutOfRange)
        )
    }

    // Money Ops
    #[test]
    fn test_valid_123_45_int() {
        assert_eq!(Money::from(12345), Money(12345))
    }

    #[test]
    fn test_valid_123_451_int() {
        assert_eq!(Money::from(123451), Money(123451))
    }

    #[test]
    fn test_valid_123_454_int() {
        assert_eq!(Money::from(123454), Money(123454))
    }

    #[test]
    fn test_valid_1234567890_int() {
        assert_eq!(Money::from(1234567890), Money(1234567890))
    }

    #[test]
    fn test_valid_12345678901234567_int() {
        assert_eq!(
            Money::from(12345678901234567),
            Money(12345678901234567)
        )
    }

    #[test]
    fn test_valid_neg_12345678901234567_int() {
        assert_eq!(
            Money::from(-12345678901234567),
            Money(-12345678901234567)
        )
    }

    #[test]
    fn test_valid_neg_123456_78_int() {
        assert_eq!(Money::from(-12345678), Money(-12345678))
    }
}
