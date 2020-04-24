use std::{fmt, str};
use std::num::ParseIntError;

mod parser;

// TODO: should this be [u8; 8], and called Bytes, instead?
pub type Inner = i64;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Money(Inner);

impl Money {
    pub const MIN_INNER: Inner = -9223372036854775808;
    pub const MAX_INNER: Inner = 9223372036854775807;

    pub const fn min() -> Money  {
        Money(Money::MIN_INNER)
    }

    pub const fn max() -> Money  {
        Money(Money::MAX_INNER)
    }

    pub const fn none() -> Money {
        Money(0)
    }
}

impl fmt::Debug for Money {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}.{}", self.0 / 100, self.0 % 100)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}.{}", self.0 / 100, self.0 % 100)
    }
}

impl str::FromStr for Money {
    type Err = ParseIntError;

    fn from_str(money_str: &str) -> Result<Self, Self::Err> {
        Money::parse_str(money_str)
    }
}

impl Default for Money {
    #[inline]
    fn default() -> Self {
        Money::none()
    }
}


#[cfg(test)]
mod tests {
    use crate::Money;

    #[test]
    fn test_money_default() {
        let default_uuid = Money::default();
        let nil_uuid = Money::none();

        assert_eq!(default_uuid, nil_uuid);
    }

    #[test]
    fn test_money_display() {
        let money = Money(1337);
        let s = money.to_string();
        assert_eq!(s, "$13.37");
    }

    #[test]
    fn test_playground() {
        let test = "14599999";
        let len = test.len() as u32;
        println!("{}", 1459 / 1000);

        let s = "-10000.32";
        println!("{:?}", s.split("-"));

        let v: Vec<&str> = s.split(|c: char| !c.is_numeric()).collect();
        println!("{:?}", v);

        assert_eq!(14599999 / 10_i32.pow(len - 2), 14);
        // assert_eq!(9223372036854775807 as f32, 92233720368547758.07)
    }
}
