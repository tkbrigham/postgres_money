use std::{fmt, str};
mod parser;

// TODO: should this be [u8; 8] instead?
pub type Bytes = i64;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Money(Bytes);

impl Money {
    /// Returns an array of 8 octets containing the Money data.
    pub const fn as_bytes(&self) -> &Bytes {
        &self.0
    }

    pub const fn from_bytes(bytes: Bytes) -> Money {
        Money(bytes)
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

// impl str::FromStr for Money {
//     fn from_str(money_str: &str) -> Result<Self, Self::Err> {
//         Money::parse_str(money_str)
//     }
// }

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
        // assert_eq!(9223372036854775807 as f32, 92233720368547758.07)
    }
}
