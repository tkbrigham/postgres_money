use std::{fmt, str};

mod parser;
use parser::Error;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Money(Inner);
pub type Inner = i64;

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
        write!(f, "${}.{}", self.0 / 100, self.0.abs() % 100)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}.{}", self.0 / 100, self.0.abs() % 100)
    }
}

impl str::FromStr for Money {
    type Err = Error;

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
}
