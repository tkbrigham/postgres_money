use std::{fmt, str};

mod parser;
mod error;

use error::Error;
use std::ops::Add;

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

    fn inner(&self) -> Inner {
        self.0
    }

    fn dollars(&self) -> String {
        format!("{}", self.0 /100)
    }

    fn cents(&self) -> String {
        let n = self.0.abs() % 100;
        let mut zero_pad = "";
        if n < 10 {
            zero_pad = "0"
        }
        format!("{}{}", zero_pad, n)
    }
}

impl fmt::Debug for Money {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}.{}", self.dollars(), self.cents())
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}.{}", self.dollars(), self.cents())
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

impl Add for Money {
    type Output = Self;

    // TODO: should this return a Result and do checked_add instead of panic?
    fn add(self, rhs: Money) -> Self::Output {
        Money(self.inner() + rhs.inner())
    }
}




#[cfg(test)]
mod tests {
    use crate::Money;

    #[test]
    fn test_money_default() {
        let default_money = Money::default();
        let nil_money = Money::none();

        assert_eq!(default_money, nil_money);
    }

    #[test]
    fn test_addition_success() {
        assert_eq!(Money(1) + Money(1), Money(2))
    }

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn test_addition_failure_overflow_max() {
        Money::max() + Money(1);
    }

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn test_addition_failure_overflow_min() {
        Money::min() + Money(-1);
    }

    // #[test]
    // fn test_subtraction() {
    //
    // }
    //
    // #[test]
    // fn test_division() {
    //
    // }
    //
    // #[test]
    // fn test_multiplication() {
    //
    // }

    // SELECT m + '123' FROM money_data;
    // SELECT m + '123.45' FROM money_data;
    // SELECT m - '123.45' FROM money_data;
    // SELECT m / '2'::money FROM money_data;
    // SELECT m * 2 FROM money_data;
    // SELECT 2 * m FROM money_data;
    // SELECT m / 2 FROM money_data;

    // -- All true
    // SELECT m = '$123.00' FROM money_data;
    // SELECT m != '$124.00' FROM money_data;
    // SELECT m <= '$123.00' FROM money_data;
    // SELECT m >= '$123.00' FROM money_data;
    // SELECT m < '$124.00' FROM money_data;
    // SELECT m > '$122.00' FROM money_data;
    //
    // -- All false
    // SELECT m = '$123.01' FROM money_data;
    // SELECT m != '$123.00' FROM money_data;
    // SELECT m <= '$122.99' FROM money_data;
    // SELECT m >= '$123.01' FROM money_data;
    // SELECT m > '$124.00' FROM money_data;
    // SELECT m < '$122.00' FROM money_data;
}
